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
//use ink_prelude::vec::Vec;
pub use self::vault::VaultManager;

#[ink::contract]
mod vault {

    use alloc::string::String;

    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},

    };

    use erc20::Erc20;
    use org::OrgManager;

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Transfer {
        transfer_id:u64,
        transfer_direction:u64,// 1: 国库转出，2:国库转入
        token_name: String,
        from_address:AccountId,
        to_address:AccountId,
        value: u64,
        transfer_time:u64,
    }





    #[ink(storage)]
    pub struct VaultManager {

        tokens: StorageHashMap<AccountId, AccountId>,
        visible_tokens: StorageHashMap<AccountId, AccountId>,
        transfer_history:StorageHashMap<u64,Transfer>,
        org_contract_address:AccountId,
        vault_contract_address:AccountId,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        InvalidTransferRecord,
    }


    #[ink(event)]
    pub struct AddVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }




    #[ink(event)]
    pub struct RemoveVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }



    #[ink(event)]
    pub struct GetTokenBalanceEvent {
        #[ink(topic)]
        token_address:AccountId,

        #[ink(topic)]
        balance:u64,
    }

    #[ink(event)]
    pub struct DepositTokenEvent {

        #[ink(topic)]
        token_name:String,
        #[ink(topic)]
        from_address:AccountId,

        #[ink(topic)]
        balance:u64,
    }


    #[ink(event)]
    pub struct WithdrawTokenEvent {
        #[ink(topic)]
        token_name:String,

        #[ink(topic)]
        to_address:AccountId,

        #[ink(topic)]
        balance:u64,
    }





    impl VaultManager {

        #[ink(constructor)]
        pub fn new(org_contract_address: AccountId) -> Self {

            let vault_contract_address = Self::env().account_id();

            Self {
                org_contract_address:org_contract_address,
                tokens: StorageHashMap::default(),
                visible_tokens: StorageHashMap::default(),
                transfer_history: StorageHashMap::default(),
                vault_contract_address: vault_contract_address,


            }
        }



        // 由合约地址获取erc20 实例
        pub fn get_erc20_by_address(&self, address:AccountId) -> Erc20 {
            let  erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(address);
            erc20_instance

        }

         // 由合约地址获取OrgManager 实例
        pub fn get_orgmanager_by_address(&self, address:AccountId) -> OrgManager {
            let  org_instance: OrgManager = ink_env::call::FromAccountId::from_account_id(address);
            org_instance

        }

        // 国库操作的权限检查
        #[ink(message)]
        pub fn check_authority(&self, caller:AccountId) -> bool {
            //return true;
            let  org = self.get_orgmanager_by_address(self.org_contract_address);

            let creator = org.get_dao_creator();
            let moderator_list = org.get_dao_moderator_list();

            if caller == creator {
                return true;
            }
            for key in moderator_list {
                let moderator = key;
                if caller == moderator {
                    return true;
                }
            }
            return false;

        }



        #[ink(message)]
        pub fn add_vault_token(&mut self,erc_20_address:AccountId) -> bool  {

            let caller = self.env().caller();

            // 国库权限控制: 只有管理员或者creator 可以增加 token

             let can_operate = self.check_authority(caller);


            if can_operate == false {
                return false;
            }


            match self.tokens.insert(
                                     erc_20_address,self.vault_contract_address
            ) {

                // 该token 已经存在，加入报错
                Some(_) => { false},
                None => {
                    self.visible_tokens.insert(
                                               erc_20_address,self.vault_contract_address);



                    self.env().emit_event(AddVaultTokenEvent{
                        token_address:erc_20_address,
                        });
                    true
                }
            }
        }


        #[ink(message)]
        // 移除token，只是从 token可见列表(visible_tokens)中移除，在tokens中该币仍然存在。
        pub fn remove_vault_token(&mut self,erc_20_address: AccountId) -> bool  {

            // 国库权限控制: 只有管理员或者creator 可以移除 token
            let caller = self.env().caller();
            let can_operate = self.check_authority(caller);

            if can_operate == false {
                return false;
            }

            match self.visible_tokens.take(&erc_20_address) {
                // 该成员不存在，移除报错
                None => { false}
                Some(_) => {

                    self.env().emit_event(RemoveVaultTokenEvent{
                        token_address:erc_20_address,
                        });
                    true
                }
            }
        }


        #[ink(message)]
        pub fn get_token_list(&self) -> ink_prelude::vec::Vec<AccountId> {
            self.visible_tokens.keys();
            let mut v:ink_prelude::vec::Vec<AccountId> = ink_prelude::vec::Vec::new();
            for key in self.visible_tokens.keys() {
                v.push(*key)
            }
            v
        }



        #[ink(message)]
        pub fn get_balance_of(&self,erc_20_address: AccountId) -> u64 {

            // 只允许查询 “注册tokens 列表” 中的 erc20 token 的余额
            if self.tokens.contains_key(&erc_20_address) {

               // let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let  erc_20 = self.get_erc20_by_address(erc_20_address);
                //let token_name = (&erc_20).name();
                let balanceof = erc_20.balance_of(self.vault_contract_address);


                self.env().emit_event(GetTokenBalanceEvent{
                    token_address:erc_20_address,
                    balance:balanceof,});

                balanceof

            } else{
                0
            }
        }


        #[ink(message)]
        // 把资金存入国库，目前只允许 往 “注册tokens 列表” 里 的 币转账。
        pub fn deposit(&mut self, erc_20_address:AccountId, from_address:AccountId,value:u64) -> bool {

            let to_address = self.vault_contract_address;

            if self.tokens.contains_key(&erc_20_address) {

                let  balanceof =  self.get_balance_of(erc_20_address);


                //let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);

                let token_name = (&erc_20).name();

                erc_20.transfer_from(from_address,to_address, value);

                // 记录转账历史

                let transfer_id:u64 = (self.transfer_history.len()+1).into();

                let transfer_time: u64 = self.env().block_timestamp();


                self.transfer_history.insert(transfer_id,
                                             Transfer{

                                                 transfer_direction:2,// 1: 国库转出，2:国库转入
                                                 token_name:token_name.clone(),
                                                 transfer_id:transfer_id,
                                                 from_address:from_address,
                                                 to_address:to_address,
                                                 value,
                                                 transfer_time});


                self.env().emit_event(DepositTokenEvent{
                    token_name: token_name.clone(),
                    from_address:from_address,
                    balance:balanceof,});

                true

            } else{
                false
            }
        }



        #[ink(message)]
        // 把资金转出国库，目前只允许 从 “可见 tokens 列表” 里的币 转出。同时， 只有管理员或者creator ,可以转出资金。
        pub fn withdraw(&mut self,erc_20_address:AccountId,to_address:AccountId,value:u64) -> bool {

            let from_address = self.vault_contract_address;

            if self.visible_tokens.contains_key(&erc_20_address) {


                // 国库权限控制: 只有管理员或者creator ,可以转出资金
                let caller = self.env().caller();
                let can_operate = self.check_authority(caller);

                if can_operate == false {
                    return false;
                }


                let  balanceof =  self.get_balance_of(erc_20_address);


                //let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);

                let token_name = (&erc_20).name();

                //erc_20.transfer_from(from_address,to_address, value);
                erc_20.transfer(to_address, value);



                // 记录转账历史
                let transfer_id:u64 = (self.transfer_history.len()+1).into();

                let transfer_time: u64 = self.env().block_timestamp();

                self.transfer_history.insert(transfer_id,
                                             Transfer{
                                                 transfer_direction:1,// 1: 国库转出，2:国库转入
                                                 token_name: token_name.clone(),
                                                 transfer_id:transfer_id,
                                                 from_address:from_address,
                                                 to_address:to_address,
                                                 value:value,
                                                 transfer_time:transfer_time});




                self.env().emit_event(WithdrawTokenEvent{
                    token_name: token_name.clone(),
                    to_address:to_address,
                    balance:balanceof,});

                true

            } else{
                false
            }
        }



        #[ink(message)]
        pub fn get_transfer_history(&self) -> ink_prelude::vec::Vec<Transfer> {
            let mut temp_vec = ink_prelude::vec::Vec::new();
            let mut iter = self.transfer_history.values();
            let mut temp = iter.next();
            while temp.is_some() {
                temp_vec.push(temp.unwrap().clone());
                temp = iter.next();
            }
            temp_vec
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
            assert_eq!(vault_manager.org_id, 1);
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
