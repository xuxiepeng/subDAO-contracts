#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
pub use self::org::OrgManager;
use ink_lang as ink;

#[ink::contract]
mod org {

    use alloc::string::String;
    use ink_prelude::collections::BTreeMap;
    use ink_prelude::vec::Vec;
    use ink_storage::collections::HashMap as StorageHashMap;

    use auth::Auth;
    #[ink(storage)]
    pub struct OrgManager {
        moderators: StorageHashMap<AccountId, String>,
        members: StorageHashMap<AccountId, String>,
        applying_members: StorageHashMap<AccountId, String>,
        owner: AccountId,
        org_id: u64,
        can_free_add_member: bool,
        is_member: bool,
        is_moderator: bool,
        is_owner: bool,
        auth_contract_address: AccountId,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        DaoModeratorExistAlready,
        DaoMemberExistAlready,
        DaoMemberNotExist,
        DaoModeratorNotExist,
    }

    #[ink(event)]
    pub struct AddDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        org_id: u64,
    }

    #[ink(event)]
    pub struct RemoveDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        org_id: u64,
    }

    #[ink(event)]
    pub struct AddDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id: u64,
    }

    #[ink(event)]
    pub struct RemoveDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id: u64,
    }

    #[ink(event)]
    pub struct ApplyDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id: u64,
    }

    #[ink(event)]
    pub struct ApproveDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id: u64,
        #[ink(topic)]
        approver: AccountId,
    }

    #[derive(Debug, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo,))]
    pub struct PageResult<T> {
        pub success: bool,
        pub err: String,
        pub total: u64,
        pub pages: u64,
        pub page: u64,
        pub size: u64,
        pub data: Vec<T>,
    }

    impl OrgManager {
        #[ink(constructor)]
        pub fn new(_owner: AccountId, org_id: u64, auth_contract_address: AccountId) -> Self {
            Self {
                org_id: org_id,
                owner: _owner,
                moderators: StorageHashMap::default(),
                members: StorageHashMap::default(),
                applying_members: StorageHashMap::default(),
                auth_contract_address: auth_contract_address,
                can_free_add_member: false,
                is_member: false,
                is_moderator: false,
                is_owner: false,
            }
        }

        fn cal_pages(&self, page: u64, size: u64, total: u64) -> (u64, u64, u64) {
            let start = page * size;
            let mut end = start + size;
            if end > total {
                end = total
            }
            assert!(size <= 0 || start >= total || start < end, "wrong params");
            let mut pages = total / size;
            if total % size > 0 {
                pages += 1;
            }
            (start, end, pages)
        }

        fn get_account_list(
            &self,
            data: &StorageHashMap<AccountId, String>,
            page: u64,
            size: u64,
        ) -> PageResult<AccountId> {
            let total = data.len() as u64;

            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut data_vec: Vec<AccountId> = Vec::new();

            for i in start..end {
                let key = data.keys().nth(i as usize);
                if let Some(s) = key {
                    data_vec.push(s.clone());
                }
            }
            return PageResult {
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: data_vec,
            };
        }

        fn get_detail_list(
            &self,
            data: &StorageHashMap<AccountId, String>,
            page: u64,
            size: u64,
        ) -> PageResult<(AccountId, String)> {
            let total = data.len() as u64;

            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut data_vec: Vec<(AccountId, String)> = Vec::new();

            for i in start..end {
                let key = data.keys().nth(i as usize);
                if let Some(s) = key {
                    let value = data.get(s).unwrap().clone();
                    data_vec.push((s.clone(), value));
                }
            }
            return PageResult {
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: data_vec,
            };
        }

        #[ink(message)]
        pub fn get_dao_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_can_free_add_member(&self) -> bool {
            self.can_free_add_member
        }

        #[ink(message)]
        pub fn set_can_free_add_member(&mut self, can_free_add_member: bool) -> bool {
            self.can_free_add_member = can_free_add_member;
            self.can_free_add_member
        }

        pub fn get_auth_by_address(&self, address: AccountId) -> Auth {
            let auth_instance: Auth = ink_env::call::FromAccountId::from_account_id(address);
            auth_instance
        }

        #[ink(message)]
        pub fn get_orgid(&self) -> u64 {
            self.org_id
        }

        #[ink(message)]
        pub fn get_dao_size(&self) -> u64 {
            let mut v: Vec<AccountId> = Vec::new();

            for key in self.moderators.keys() {
                v.push(*key)
            }

            for key in self.members.keys() {
                v.push(*key)
            }

            // remove duplicated account
            v.sort_unstable();
            v.dedup();

            return v.len() as u64;
        }

        #[ink(message)]
        pub fn get_dao_moderator_list(&self, page: u64, size: u64) -> PageResult<AccountId> {
            return self.get_account_list(&self.moderators, page, size);
        }

        #[ink(message)]
        pub fn get_dao_members_list(&self, page: u64, size: u64) -> PageResult<AccountId> {
            return self.get_account_list(&self.members, page, size);
        }

        #[ink(message)]
        pub fn get_dao_moderator_detail_list(
            &self,
            page: u64,
            size: u64,
        ) -> PageResult<(AccountId, String)> {
            return self.get_detail_list(&self.moderators, page, size);
        }

        #[ink(message)]
        pub fn get_dao_member_detail_list(
            &self,
            page: u64,
            size: u64,
        ) -> PageResult<(AccountId, String)> {
            return self.get_detail_list(&self.members, page, size);
        }

        #[ink(message)]
        pub fn get_dao_apply_member_detail_list(
            &self,
            page: u64,
            size: u64,
        ) -> PageResult<(AccountId, String)> {
            return self.get_detail_list(&self.applying_members, page, size);
        }

        // FIXME: this implementation is incorrect when the added one is already a member of the dao.
        #[ink(message)]
        pub fn add_dao_moderator(&mut self, name: String, moderator: AccountId) -> bool {
            let caller = self.env().caller();

            if caller != self.owner {
                return false;
            }

            let mut auth_instance = self.get_auth_by_address(self.auth_contract_address);
            // assert!(self.owner == caller || self._has_permission(caller, String::from("auth"),String::from("grant")));
            auth_instance.grant_permission(moderator, String::from("vote"), String::from("new"));
            auth_instance.grant_permission(moderator, String::from("vote"), String::from("vote"));

            match self.moderators.insert(moderator, name) {
                Some(_) => false,
                None => {
                    let org_id = self.org_id;
                    self.env()
                        .emit_event(AddDAOModeratorEvent { moderator, org_id });
                    true
                }
            }
        }

        #[ink(message)]
        pub fn add_dao_moderator_without_grant(
            &mut self,
            name: String,
            moderator: AccountId,
        ) -> bool {
            let caller = self.env().caller();

            if caller != self.owner {
                return false;
            }

            match self.moderators.insert(moderator, name) {
                Some(_) => false,
                None => {
                    let org_id = self.org_id;
                    self.env()
                        .emit_event(AddDAOModeratorEvent { moderator, org_id });
                    true
                }
            }
        }

        // FIXME: this implementation is incorrect when the added one is already a moderator of the dao.
        #[ink(message)]
        pub fn add_dao_member(&mut self, name: String, member: AccountId) -> bool {
            let (_is_member, is_moderator, is_owner) = self.who_am_i();

            // let caller = self.env().caller();

            if self.can_free_add_member == false && !is_moderator && !is_owner {
                return false;
            }

            let mut auth_instance = self.get_auth_by_address(self.auth_contract_address);
            // assert!(auth_instance.has_permission(caller, String::from("auth"),String::from("grant")));
            auth_instance.grant_permission(member, String::from("vote"), String::from("vote"));

            match self.members.insert(member, name) {
                Some(_) => false,
                None => {
                    let org_id = self.org_id;
                    self.env().emit_event(AddDAOMemberEvent { member, org_id });
                    true
                }
            }
        }

        pub fn add_dao_member_private(&mut self, name: String, member: AccountId) -> bool {
            // let caller = self.env().caller();
            let mut auth_instance = self.get_auth_by_address(self.auth_contract_address);
            // assert!(auth_instance.has_permission(caller, String::from("auth"),String::from("grant")));
            auth_instance.grant_permission(member, String::from("vote"), String::from("vote"));

            match self.members.insert(member, name) {
                Some(_) => false,
                None => {
                    let org_id = self.org_id;
                    self.env().emit_event(AddDAOMemberEvent { member, org_id });
                    true
                }
            }
        }

        #[ink(message)]
        pub fn batch_add_dao_member(&mut self, members: BTreeMap<String, AccountId>) -> bool {
            for (name, account_id) in &members {
                self.add_dao_member(String::from(name), *account_id);
            }
            true
        }

        #[ink(message)]
        pub fn remove_dao_moderator(&mut self, member: AccountId) -> bool {
            let caller = self.env().caller();

            if caller != self.owner {
                return false;
            }

            match self.moderators.take(&member) {
                None => false,
                Some(_) => {
                    let org_id = self.org_id;
                    self.env().emit_event(RemoveDAOModeratorEvent {
                        moderator: member,
                        org_id,
                    });
                    true
                }
            }
        }

        // FIXME: This implementation is incorrect!!! Need re-factor this part, only moderators can remove members
        #[ink(message)]
        pub fn remove_dao_member(&mut self, member: AccountId) -> bool {
            match self.members.take(&member) {
                None => false,
                Some(_) => {
                    let org_id = self.org_id;
                    self.env().emit_event(RemoveDAOMemberEvent {
                        member: member,
                        org_id: org_id,
                    });
                    true
                }
            }
        }

        // #[ink(message)]
        // pub fn resign(&mut self,member: AccountId) -> bool  {

        //     if self.members.contains_key(&member) {
        //         self.members.take(&member);
        //         return true;
        //     };

        //     if self.moderators.contains_key(&member) {
        //         self.moderators.take(&member);
        //         return true;
        //     };
        //     return false;
        // }

        #[ink(message)]
        pub fn resign_member(&mut self, member: AccountId) -> bool {
            if self.members.contains_key(&member) {
                self.members.take(&member);
                return true;
            };

            return false;
        }

        #[ink(message)]
        pub fn resign_moderator(&mut self, moderator: AccountId) -> bool {
            if self.moderators.contains_key(&moderator) {
                self.moderators.take(&moderator);
                return true;
            };
            return false;
        }

        #[ink(message)]
        pub fn who_am_i(&mut self) -> (bool, bool, bool) {
            let caller = self.env().caller();

            if self.members.contains_key(&caller) {
                self.is_member = true;
            } else {
                self.is_member = false;
            }

            if self.moderators.contains_key(&caller) {
                self.is_moderator = true;
            } else {
                self.is_moderator = false;
            }

            if caller == self.owner {
                self.is_owner = true;
            } else {
                self.is_owner = false;
            }

            return (self.is_member, self.is_moderator, self.is_owner);
        }

        #[ink(message)]
        pub fn check_role_by_account(&self, user: AccountId) -> (bool, bool, bool) {
            let mut is_member = false;
            let mut is_moderator = false;
            let mut is_owner = false;
            if self.members.contains_key(&user) {
                is_member = true;
            }

            if self.moderators.contains_key(&user) {
                is_moderator = true;
            }

            if user == self.owner {
                is_owner = true;
            }

            return (is_member, is_moderator, is_owner);
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> bool {
            let caller = self.env().caller();

            // only owner can transfer the ownership of the org
            if caller != self.owner {
                return false;
            }

            if self.members.contains_key(&new_owner) {
                self.add_dao_moderator(self.members.get(&new_owner).unwrap().clone(), new_owner);
                self.remove_dao_member(new_owner);
            } else if !self.moderators.contains_key(&new_owner) {
                return false;
            }

            self.owner = new_owner;
            return true;
        }
        #[ink(message)]
        pub fn apply_member(&mut self, name: String, member: AccountId) -> bool {
            match self.applying_members.insert(member, name) {
                Some(_) => false,
                None => {
                    let org_id = self.org_id;
                    self.env()
                        .emit_event(ApplyDAOMemberEvent { member, org_id });
                    true
                }
            }
        }

        pub fn check_authority(&self, caller: AccountId) -> bool {
            if caller == self.owner {
                return true;
            }

            if self.moderators.contains_key(&caller) {
                return true;
            }
            return false;
        }

        #[ink(message)]
        pub fn approve_member(&mut self, name: String, member: AccountId) -> bool {
            let caller = self.env().caller();

            let can_operate = self.check_authority(caller);

            if can_operate == false {
                return false;
            }

            if self.applying_members.contains_key(&member) {
                let caller_new = self.env().caller();
                self.add_dao_member_private(name, member);
                self.applying_members.take(&member);
                let org_id = self.org_id;

                self.env().emit_event(ApproveDAOMemberEvent {
                    member,
                    org_id,
                    approver: caller_new,
                });

                return true;
            };
            return false;
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{call, test};
        use ink_lang as ink;

        #[ink::test]
        fn new_org_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: need auth contract here, use alice instead to solve compile issue.
            let mut org_manager = OrgManager::new(accounts.alice, 1, accounts.alice);

            assert_eq!(org_manager.owner, accounts.alice);
            assert_eq!(org_manager.org_id, 1);
        }

        #[ink::test]
        fn add_member_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice, 1, accounts.alice);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name, accounts.bob);
            let mut member = org_manager
                .members
                .keys()
                .into_iter()
                .next()
                .unwrap()
                .clone();
            assert_eq!(member, accounts.bob);
        }

        #[ink::test]
        fn add_moderator_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice, 1, accounts.alice);
            let bob_name = String::from("bob");
            org_manager.add_dao_moderator(bob_name, accounts.bob);
            let mut member = org_manager
                .moderators
                .keys()
                .into_iter()
                .next()
                .unwrap()
                .clone();
            assert_eq!(member, accounts.bob);
        }

        #[ink::test]
        fn remove_moderator_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice, 1, accounts.alice);
            let bob_name = String::from("bob");
            org_manager.add_dao_moderator(bob_name, accounts.bob);
            org_manager.remove_dao_moderator(accounts.bob);

            // let mut members = org_manager.get_dao_moderator_list();
            assert_eq!(org_manager.moderators.len(), 0);
        }

        #[ink::test]
        fn remove_members_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice, 1, accounts.alice);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name, accounts.bob);
            org_manager.remove_dao_member(accounts.bob);
            assert_eq!(org_manager.members.len(), 0);
        }

        #[ink::test]
        fn resign_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice, 1, accounts.alice);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name, accounts.bob);
            let eve_name = String::from("eve");
            org_manager.add_dao_member(eve_name, accounts.eve);
            assert_eq!(org_manager.members.len(), 2);
            org_manager.resign_member(accounts.bob);
            assert_eq!(org_manager.members.len(), 1);
            org_manager.resign_member(accounts.eve);
            assert_eq!(org_manager.members.len(), 0);
        }
    }
}
