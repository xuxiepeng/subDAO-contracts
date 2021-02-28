#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::github::Github;

#[ink::contract]
mod github {

    // use ink_prelude::vec::Vec; 
    // use ink_prelude::string::String;

    use alloc::string::String;
    
    // #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    // use ink_prelude::string;

    type Index = u64;

    #[derive(scale::Encode,scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct PullRequest {
        repo_url: String,
        pr_number: u64,
        github_id: u64,
        account_id: AccountId,
    }

    #[ink(storage)]
    pub struct Github {
        length: u64,
        pullrequests: StorageHashMap<Index, PullRequest>,
        auditorresults: StorageHashMap<(Index, AccountId), bool>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if caller is not auditor.
        CallerIsNotAuditor,
        PRIsNotRegisted,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct NewPR{
        #[ink(topic)]
        index: Index,

        #[ink(topic)]
        creator: AccountId,

        #[ink(topic)]
        auditor: AccountId,
    }

    #[ink(event)]
    pub struct AuditPR{
        #[ink(topic)]
        index: Index,

        #[ink(topic)]
        auditor: AccountId,

        #[ink(topic)]
        result: bool,
    }

    impl Github {

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }
       
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                length:0,
                pullrequests: StorageHashMap::default(),
                auditorresults: StorageHashMap::default(),
             }
        }

        #[ink(message)]
        pub fn new_pull_request_auditor(& mut self, repo_url: String, pr_number: u64, github_id: u64, account_id: AccountId, auditor_id: AccountId) -> Result<()> {
            let index = self.length.clone() + 1;
            self.length += 1;
            
            let pr = PullRequest{
                repo_url: repo_url,
                pr_number: pr_number,
                github_id: github_id,
                account_id: account_id,
            };
            self.pullrequests.insert(index, pr);
            self.auditorresults.insert((index,auditor_id),false);

            self.env().emit_event(NewPR{
                index,
                creator:account_id,
                auditor:auditor_id,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn query_pull_request_audit_status(&self, index: Index ) -> bool{
            let mut res = false;
           for ((_index, _account_id),_auditor_result) in &self.auditorresults {
               if _index == &index {
                 res =  *_auditor_result;
                 break;
               }
           }
           res
        }

        #[ink(message)]
        pub fn audit_pull_request(& mut self, index: Index, audit_result: bool ) -> Result<()> {
            let caller = self.env().caller();

            if !self.pullrequests.contains_key(&index) {
                return Err(Error::PRIsNotRegisted)
            }

            if !self.auditorresults.contains_key (&(index,caller)) {
                return Err(Error::CallerIsNotAuditor)
            }

            self.auditorresults.insert((index,caller),audit_result);
            self.env().emit_event(AuditPR{
                index,
                auditor:caller,
                result: audit_result,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_current_index(&self) -> u64 {
            self.length
        }
    }


    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink_env::{
            call,
            test,
        };
        use ink_lang as ink;

        const DEFAULT_CALLEE_HASH: [u8; 32] = [0x07; 32];
        const DEFAULT_ENDOWMENT: Balance = 1_000_000;
        const DEFAULT_GAS_LIMIT: Balance = 1_000_000;

        fn set_next_caller(caller: AccountId) {
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                caller,
                AccountId::from(DEFAULT_CALLEE_HASH),
                DEFAULT_ENDOWMENT,
                DEFAULT_GAS_LIMIT,
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])),
            )
        }

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let github = Github::default();
            assert_eq!(github.get_current_index(), 0);
        }

        // We test a simple use case of our contract.
        #[test]
        fn it_works() {

            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            let mut github = Github::new();
            // repo_url: Hash, pr_number: u64, github_id: u64, account_id: AccountId, auditor_id: AccountId
            // repo_url: https://github.com/paritytech/ink
            // pr_number: 702
            // github_id: test 123456
            // accouont_id: alice
            // auditor_id: bob
            github.new_pull_request_auditor("https://github.com/paritytech/ink".to_string(), 702, 123456, accounts.alice, accounts.bob);
            assert_eq!(github.get_current_index(), 1);
            assert_eq!(github.query_pull_request_audit_status(github.get_current_index()), false);
            set_next_caller(accounts.bob);
            // assert_eq!(ink_env::caller(),accounts.bob);
            assert_eq!(github.audit_pull_request(github.get_current_index(), true),Ok(()));
            assert_eq!(github.query_pull_request_audit_status(github.get_current_index()), true);
        }

        #[test]
        fn caller_is_not_auditor() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            let mut github = Github::new();
            github.new_pull_request_auditor("https://github.com/paritytech/ink".to_string(), 702, 123456, accounts.alice, accounts.bob);
            set_next_caller(accounts.charlie);
            // assert_eq!(ink_env::caller(),accounts.bob);
            assert_eq!(github.audit_pull_request(github.get_current_index(), true),Err(Error::CallerIsNotAuditor));
            assert_eq!(github.query_pull_request_audit_status(github.get_current_index()), false);
        }

        #[test]
        fn pr_is_not_new() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            let mut github = Github::new();
            github.new_pull_request_auditor("https://github.com/paritytech/ink".to_string(), 702, 123456, accounts.alice, accounts.bob);
            set_next_caller(accounts.charlie);
            assert_eq!(github.audit_pull_request(github.get_current_index()+10, true),  Err(Error::PRIsNotRegisted));
        }
    }
}
