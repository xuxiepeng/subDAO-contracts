// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
use alloc::vec::Vec;
pub use self::vault::VaultManager;

#[ink::contract]
mod vault {
    use alloc::string::String;

    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
        
    };

    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,Default
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Transfer {
        transfer_id:u64,
        from_address:AccountId,
        to_address:AccountId,
        value: Balance,
        transfer_time:u64,
    }



    #[ink(storage)]
    pub struct VaultManager {

        tokens: StorageHashMap<AccountId, String>,
        token_balances: StorageHashMap<AccountId, Balance>,
        visible_tokens: StorageHashMap<AccountId, String>,
        transfer_history:StorageHashMap<u64,Transfer>,
        orgId:u64,
    }



     /// Errors that can occur upon calling this contract.
     #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
     #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
     pub enum Error {
        InvalidTransferRecord,
     }

    
    #[ink(event)]
    pub struct addVaultTokenEvent {
        #[ink(topic)]
        tokenAddress: AccountId,
        #[ink(topic)]
        orgId:u64,
    }



    
    #[ink(event)]
    pub struct removeVaultTokenEvent {
        #[ink(topic)]
        tokenAddress: AccountId,
        #[ink(topic)]
        orgId:u64,
    }



    #[ink(event)]
    pub struct getTokenBalanceEvent {
        #[ink(topic)]
        tokenAddress:AccountId,
        #[ink(topic)]
        orgId:u64,
        #[ink(topic)]
        balance:Balance,
    }

    #[ink(event)]
    pub struct depositTokenEvent {
        #[ink(topic)]
        tokenAddress:AccountId,
        #[ink(topic)]
        fromAddress:AccountId,
        #[ink(topic)]
        orgId:u64,
        #[ink(topic)]
        balance:Balance,
    }


    #[ink(event)]
    pub struct withdrawTokenEvent {
        #[ink(topic)]
        tokenAddress:AccountId,
        #[ink(topic)]
        toAddress:AccountId,
        #[ink(topic)]
        orgId:u64,
        #[ink(topic)]
        balance:Balance,
    }

    



    impl VaultManager {

        #[ink(constructor)]
        pub fn new(_orgId:u64) -> Self {
            Self {
                
                orgId:_orgId,
                tokens: StorageHashMap::default(),
                token_balances: StorageHashMap::default(),
                visible_tokens: StorageHashMap::default(),
                transfer_history: StorageHashMap::default(),

            }
        }

        

        #[ink(message)]
        pub fn add_vault_token(&mut self,token_name:String,token_address: AccountId) -> bool  {
            let caller = self.env().caller();

            let clone_token_name = token_name.clone();
            match self.tokens.insert(token_address,token_name) {
                
                // 该token 已经存在，加入报错
                Some(_) => { false},
                None => {
                    self.visible_tokens.insert(token_address,clone_token_name);
                    self.token_balances.insert(token_address,0);

                    let orgId = self.orgId;
                    self.env().emit_event(addVaultTokenEvent{
                        tokenAddress:token_address,
                         orgId,});
                    true
                }
            }
        }


        #[ink(message)]
        // 移除token，只是从 token可见列表(visible_tokens)中移除，在tokens中该币仍然存在。
        pub fn remove_vault_token(&mut self,token_address: AccountId) -> bool  {

            match self.visible_tokens.take(&token_address) {
                // 该成员不存在，移除报错
                None => { false}
                Some(_) => {
                    let orgId = self.orgId;
                    self.env().emit_event(removeVaultTokenEvent{
                        tokenAddress:token_address,
                         orgId,});
                     true
                }
            }
        }


        #[ink(message)]
        pub fn get_token_list(&self) -> alloc::vec::Vec<AccountId> {
            self.visible_tokens.keys();
            let mut v:alloc::vec::Vec<AccountId> = alloc::vec::Vec::new();
            for key in self.visible_tokens.keys() {
                v.push(*key)
            }
            v
        }



        #[ink(message)]
        pub fn get_balance_of(&self,token_address: AccountId) -> Balance {
            if self.token_balances.contains_key(&token_address) {
               let balanceof =  self.token_balances.get(&token_address).copied().unwrap_or(0);
               let orgId = self.orgId;
               self.env().emit_event(getTokenBalanceEvent{
                   tokenAddress:token_address,
                    orgId,
                    balance:balanceof,});

                balanceof

            } else{
                0
            }
        }


        #[ink(message)]
        // 把资金存入国库
        pub fn deposit(&mut self,token_address: AccountId,from_address:AccountId,value:Balance) -> bool {
            if self.token_balances.contains_key(&token_address) {
               
               let mut balanceof =  self.token_balances.get(&token_address).copied().unwrap_or(0);
               balanceof = balanceof + value;
               self.token_balances.insert(token_address,balanceof);

               // todo: 外部账号减少金额,
               //todo:
    
               // 记录转账历史
            
               let transfer_id:u64 = (self.transfer_history.len()+1).into();
               
               let transfer_time: u64 = self.env().block_timestamp();
        
             
               self.transfer_history.insert(transfer_id, 
                Transfer{
                    transfer_id:transfer_id,
                    from_address:from_address,
                    to_address:token_address,
                    value,
                    transfer_time});

               let orgId = self.orgId;
               self.env().emit_event(depositTokenEvent{
                   tokenAddress:token_address,
                   fromAddress:from_address,
                    orgId,
                    balance:balanceof,});

                true

            } else{
                false
            }
        }



        #[ink(message)]
        // 把资金转出国库
        pub fn withdraw(&mut self,token_address: AccountId,to_address:AccountId,value:Balance) -> bool {
            if self.token_balances.contains_key(&token_address) {
               
               let mut balanceof =  self.token_balances.get(&token_address).copied().unwrap_or(0);
               balanceof = balanceof - value;
               self.token_balances.insert(token_address,balanceof);
               // todo: 外部账号增加金额,
               // todo:
               // 记录转账历史
               let transfer_id:u64 = (self.transfer_history.len()+1).into();
               
               let transfer_time: u64 = self.env().block_timestamp();

               self.transfer_history.insert(transfer_id, 
                Transfer{
                    transfer_id:transfer_id,
                    from_address:token_address,
                    to_address:to_address,
                    value:value,
                    transfer_time:transfer_time});

                   
            
               let orgId = self.orgId;
               self.env().emit_event(withdrawTokenEvent{
                   tokenAddress:token_address,
                   toAddress:to_address,
                    orgId,
                    balance:balanceof,});

                true

            } else{
                false
            }
        }




        // 转账的私有函数
        fn tranfer(&self, from_address:AccountId,to_address:AccountId, value:Balance,token_name:String) -> bool {
            // todo:
            true
        }



        #[ink(message)]
        pub fn get_transfer_history(&self) -> alloc::vec::Vec<Transfer> {

            let caller = self.env().caller();

            self.transfer_history.keys();
            let mut v:alloc::vec::Vec<Transfer> = alloc::vec::Vec::new();
            for key in self.transfer_history.keys() {

            let temp = Transfer { 
                transfer_id:0,
                    from_address: caller,
                    to_address:caller,
                    value:0,
                    transfer_time:0,
                };
            let transfer = self.transfer_history.get(&key).copied().unwrap_or(temp);
            v.push(transfer);

            }
            v
        }

    }


    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{
            call,
            test,
        };
        use ink_lang as ink;

        #[ink::test]
        fn new_vault_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            assert_eq!(vault_manager.orgId, 1);
        }

        #[ink::test]
        fn add_token_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
        }


        #[ink::test]
        fn remove_token_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.remove_vault_token(accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
            assert_eq!(vault_manager.visible_tokens.len(), 0);
        }


        #[ink::test]
        fn get_token_list_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            let dot_name = String::from("dot");
            vault_manager.add_vault_token(dot_name,accounts.alice);
            assert_eq!(vault_manager.get_token_list().len(), 2);
        }


        #[ink::test]
        fn get_balance_of_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            assert_eq!(vault_manager.get_balance_of(accounts.bob), 0);
        }



        #[ink::test]
        fn deposit_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.deposit(accounts.bob,accounts.alice,100);
            assert_eq!(vault_manager.get_balance_of(accounts.bob),100);
            
        }


        #[ink::test]
        fn withdraw_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.deposit(accounts.bob,accounts.eve,1000);
            vault_manager.withdraw(accounts.bob,accounts.alice,100);
            assert_eq!(vault_manager.get_balance_of(accounts.bob),900);
            
        }


        #[ink::test]
        fn transfer_history_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.deposit(accounts.bob,accounts.eve,1000);
            vault_manager.withdraw(accounts.bob,accounts.alice,100);
            assert_eq!(vault_manager.get_transfer_history().len(),2);
            
        }

    }
}
