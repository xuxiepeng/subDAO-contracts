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
    // TODO temp可以为空 增加合约名
    pub struct DAOTemplate {
        owner: AccountId,
        erc20_code_hash: Hash,
        dao_manager_code_hash: Hash,
        org_code_hash: Hash,
        vault_code_hash: Hash,
        vote_code_hash: Hash,
        github_code_hash: Hash,
    }

    // TODO 增加templist接口

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInstance {
        owner: AccountId,
        dao_manager: DAOManager,
        dao_manager_addr: AccountId,
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
        dao_addr: AccountId,
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
                                    erc20_name: String, erc20_symbol: String, erc20_initial_supply: u64, erc20_decimals: u8,
                                    vote_time: u64, vote_support_require_pct: u64, vote_min_require_num: u64) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            let total_balance = Self::env().balance();
            // assert_eq!(total_balance >= 20, true);

            // query template info
            let template = self.template_map.get(&index).unwrap();

            // instance dao_manager
            let dao_instance_params = DAOManager::new(controller, self.instance_index)
                .endowment(total_balance / 4)
                .code_hash(template.dao_manager_code_hash)
                .params();
            let dao_init_result = ink_env::instantiate_contract(&dao_instance_params);
            let dao_addr = dao_init_result.expect("failed at instantiating the `DAO Instance` contract");
            let mut dao_instance: DAOManager = ink_env::call::FromAccountId::from_account_id(dao_addr);

            self.env().emit_event(InstanceDAO {
                index: self.template_index,
                owner: Some(controller),
                dao_addr: dao_addr,
            });

            // init instance
            dao_instance.init(template.erc20_code_hash, erc20_name, erc20_symbol, erc20_initial_supply, erc20_decimals,
                                      template.org_code_hash,
                                      template.vault_code_hash,
                                      template.vote_code_hash, vote_time, vote_support_require_pct, vote_min_require_num,
                                      template.github_code_hash);

            self.instance_map.insert(self.instance_index, DAOInstance {
                owner: controller,
                dao_manager: dao_instance,
                dao_manager_addr: dao_addr,
            });
            self.instance_index += 1;

            true
        }
    }
}
