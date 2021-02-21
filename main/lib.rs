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

use ink_lang as ink;

#[ink::contract]
mod main {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
        // Vec as StorageVec,
    };
    // use ink_prelude::string::String;
    use dao_manager::DAOManager;

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOTemplate {
        owner: AccountId,
        erc20_code_hash: Hash,
        dao_manager_code_hash: Hash,
        org_code_hash: Hash,
        vault_code_hash: Hash,
        vote_code_hash: Hash,
        github_code_hash: Hash,
    }

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInstance {
        owner: AccountId,
        // TODO 记录dao的地址
        dao_manager: DAOManager,
    }

    #[ink(storage)]
    pub struct Main {
        owner: AccountId,
        template_index: u64,
        template_map: StorageHashMap<u64, DAOTemplate>,
        instance_index: u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
    }

    #[ink(event)]
    pub struct AddTemplate {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
    }

    #[ink(event)]
    pub struct InstanceDAO {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        dao_addr: Option<AccountId>,
    }

    impl Main {
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            let instance = Self {
                owner: controller,
                template_index: 0,
                template_map: StorageHashMap::new(),
                instance_index: 0,
                instance_map: StorageHashMap::new(),
            };
            instance
        }

        #[ink(message)]
        pub fn add_template(&mut self, erc20_code_hash: Hash, dao_manager_code_hash: Hash,
                            org_code_hash: Hash, vault_code_hash: Hash, vote_code_hash: Hash, github_code_hash: Hash) -> bool {
            assert_eq!(self.template_index + 1 > self.template_index, true);
            let from = self.env().caller();
            self.template_map.insert(self.template_index, DAOTemplate {
                owner: from,
                erc20_code_hash,
                dao_manager_code_hash,
                org_code_hash,
                vault_code_hash,
                vote_code_hash,
                github_code_hash,
            });
            self.env().emit_event(AddTemplate {
                index: self.template_index,
                owner: Some(from),
            });
            self.template_index += 1;
            true
        }

        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
            return *self.template_map.get(&index).unwrap()
        }

        #[ink(message)]
        pub fn instance_by_template(&mut self, index: u64, controller: AccountId,
                                    erc20_initial_supply: u64, erc20_decimals: u8,
                                    vote_time: u64, vote_support_require_pct: u64, vote_min_require_num: u64) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            let total_balance = Self::env().balance();
            // assert_eq!(total_balance >= 20, true);

            // query template info
            let template = self.template_map.get(&index).unwrap();

            // instance dao_manager test
            let mut dao_manager_instance = DAOManager::new(controller, self.instance_index)
                .endowment(total_balance / 4)
                .code_hash(template.dao_manager_code_hash)
                .instantiate()
                .expect("failed at instantiating the `Erc20` contract");
            self.env().emit_event(InstanceDAO {
                index: self.template_index,
                owner: Some(controller),
                dao_addr: None,
            });

            // init instance
            dao_manager_instance.init(template.erc20_code_hash, erc20_initial_supply, erc20_decimals,
                                      template.org_code_hash,
                                      template.vault_code_hash,
                                      template.vote_code_hash, vote_time, vote_support_require_pct, vote_min_require_num,
                                      template.github_code_hash);

            self.instance_map.insert(self.instance_index, DAOInstance {
                owner: controller,
                dao_manager: dao_manager_instance,
            });
            self.instance_index += 1;

            true
        }
    }
}
