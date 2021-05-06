#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::auth::Auth;

#[ink::contract]
mod auth {


    use alloc::string::String;
    use alloc::vec::Vec;
    
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
        action_id: ActionId,
        actions: StorageHashMap<(String, String),Action>,
        actions_auths: StorageHashMap<(AccountId, ActionId), Action>,
    }

    impl Auth {

        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                action_id: 0,
                actions: StorageHashMap::new(),
                actions_auths: StorageHashMap::new(),
            }
        }
        
        #[ink(message)]
        pub fn has_permission(& self, account_id: AccountId, contract_name: String, function_name: String)  -> bool {
            if let Some(action) = self.actions.get(&(contract_name, function_name)) {
                if let Some(auth) = self.actions_auths.get(&(account_id, action.action_id)) {
                    true 
                }            
            }
           return false;
        }

        #[ink(message)]
        pub fn grant_permission(& mut self, account_id: AccountId, contract_name: String, function_name: String) ->  Result<(), String> {
            assert!(self.owner == self.env().caller());
            if let Some(action) = self.actions.get(&(contract_name, function_name)){
                self.actions_auths.insert(&(account_id, action.action_id), action);
                Ok(())
           }
           Err("grant permission failed".to_string())
        }


        #[ink(message)]
        pub fn revoke_permission(& mut self,account_id: AccountId,contract_name: String, function_name: String) -> Result<(), String> {
            assert!(self.owner == self.env().caller());
            if let Some(action) = self.actions.get(&(contract_name, function_name)){
                self.actions_auths.take(&(account_id, action.action_id));
                Ok(())
           }
           Err("remove permission failed".to_string())
        }

        
        #[ink(message)]
        pub fn register_action(& mut self,contract_name: String, function_name: String, action_title: String) -> bool {
            assert!(self.owner == self.env().caller());
            let action_id = self.action_id;
            self.action_id += 1;
            let action = Action{
                action_id,
                action_title,
                contract_name,
                function_name,
            };
            self.actions.insert(&(contract_name, function_name), action);
            true
        }


        #[ink(message)]
        pub fn cancel_action(& mut self,contract_name: String, function_name: String) -> bool {
            assert!(self.owner == self.env().caller());
            self.actions.take(&(contract_name, function_name));
            true
        }

        #[ink(message)]
        pub fn show_actions_by_contract(& self, contract_name: String) -> Vec<Action> {
        
            let mut actions_vec: Vec<Action> = Vec::new();
            for ((cname, fname), val) in &self.actions {
                if  cname == contract_name {
                    actions_vec.push(&val);
                }
            }
            actions_vec
        }

        #[ink(message)]
        pub fn show_actions_by_user(& self, owner: AccountId) -> Vec<Action> {
        
            let mut actions_vec: Vec<Action> = Vec::new();
            for ((account_id, action_id), val) in &self.actions_auths {
                if account_id == owner {
                    actions_vec.push(&val)
                }
            }
            actions_vec
        }
    }

    #[cfg(test)]
    mod tests {
        use ink_lang as ink;

        use super::*;
        use ink_env::{
            call,
            test,
        };

        #[ink::test]
        fn test_grant_permission() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut auth = Auth::new(accounts.alice);
            let r = auth.grant_permission(accounts.bob, "hello".to_string(), "world".to_string());
            match r {
                Ok(()) => ink_env::debug_println("success"),
                _ => ink_env::debug_println("failed"),
            }
        }
    }

}
