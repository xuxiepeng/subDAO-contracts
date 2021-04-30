#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::auth::Auth;

#[ink::contract]
mod auth {


    use alloc::string::String;
    use ink_prelude::vec::Vec;
    
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

    type ActionId = u32;

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
    pub struct Action {
        action_id: ActionId,
        action_title: String, //used for front end display
        contract_name: String,
        function_name: String,
    }

    #[ink(storage)]
    pub struct Auth {
        owner: AccountId,
        actions: StorageHashMap<(contract_name,function_name),ActionId>,
        actions_auths: StorageHashMap<(AccountId,ActionId), Action>,
    }

    impl Auth {

        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                actions: StorageHashMap::new(),
                actions_auths: StorageHashMap::new(),
            }
        }
        
        #[ink(message)]
        pub fn has_permission(& self, account_id: AccountId,contract_name: String, function_name: String)  -> bool {
           let mut res = false;
            // todo:
           res
        }

        #[ink(message)]
        pub fn grant_permission(& mut self, account_id: AccountId,contract_name: String, function_name: String) ->  Result<()> {
           // only auth owner can perform
            // todo:
           Ok(())
        }


        #[ink(message)]
        pub fn revoke_permission(& mut self,account_id: AccountId,contract_name: String, function_name: String) -> Result<()> {
          // only auth owner can perform
           // todo:
             Ok(())
        }

        #[ink(message)]
        pub fn register_action(& mut self,contract_name: String, function_name: String) -> bool {
            // only auth owner can perform
            let mut res = false;
            // todo:
           res
        }


        #[ink(message)]
        pub fn cancel_action(& mut self,contract_name: String, function_name: String) -> bool {
            // only auth owner can perform
            let mut res = false;
            // todo:
           res
        }

        #[ink(message)]
        pub fn show_actions_by_contract(& self, contract_name: String) -> Vec<Action> {
        
            let mut actions_vec = Vec::new();
           // todo:
            actions_vec
        }

        #[ink(message)]
        pub fn show_actions_by_user(& self, owner: AccountId) -> Vec<Action> {
        
            let mut actions_vec = Vec::new();
           // todo:
            actions_vec
        }


    }

}