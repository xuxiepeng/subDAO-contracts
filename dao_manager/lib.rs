#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::dao_manager::DAOManager;

#[ink::contract]
mod dao_manager {

    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},

    };
    use base::Base;
    use erc20::Erc20;
    use org::OrgManager;
    use vault::VaultManager;
    use vote_manager::VoteManager;
    use github::Github;
    use template_manager::DAOTemplate;

    /// DAO component instances
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponents {
        base: Option<Base>,
        erc20: Option<Erc20>,
        org: Option<OrgManager>,
        vault: Option<VaultManager>,
        vote: Option<VoteManager>,
        github: Option<Github>,
    }

    /// DAO component instance addresses
    #[derive(
    Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponentAddrs {
        // base module contract's address
        base_addr: Option<AccountId>,
        // erc20 module contract's address
        erc20_addr: Option<AccountId>,
        // org module contract's address
        org_addr: Option<AccountId>,
        // vault module contract's address
        vault_addr: Option<AccountId>,
        // vote module contract's address
        vote_addr: Option<AccountId>,
        // github module contract's address
        github_addr: Option<AccountId>,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DAOManager {
        init: bool,
        controller: AccountId,
        org_id: u64,
        template: DAOTemplate,
        components: DAOComponents,
        component_addrs: DAOComponentAddrs,
    }

    impl DAOManager {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(controller: AccountId, org_id: u64, template: DAOTemplate) -> Self {
            Self {
                init: false,
                controller,
                org_id,
                template,
                components: DAOComponents {
                    base: None,
                    erc20: None,
                    org: None,
                    vault: None,
                    vote: None,
                    github: None,
                },
                component_addrs: DAOComponentAddrs {
                    base_addr: None,
                    erc20_addr: None,
                    org_addr: None,
                    vault_addr: None,
                    vote_addr: None,
                    github_addr: None,
                },
            }
        }

        #[ink(message)]
        pub fn init(&mut self, base_name: String, base_logo: String, base_desc: String,
                    erc20_name: String, erc20_symbol: String, erc20_initial_supply: u64, erc20_decimals: u8) -> bool {
            assert_eq!(self.init, false);
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);

            // init components
            let components_hash_map = self.template.components.clone();
            let base_code_hash = components_hash_map.get("BASE");
            let erc20_code_hash = components_hash_map.get("ERC20");
            let org_code_hash = components_hash_map.get("ORG");
            let vault_code_hash = components_hash_map.get("VAULT");
            let vote_code_hash = components_hash_map.get("VOTE");
            let github_code_hash = components_hash_map.get("GITHUB");
            self._init_base(base_code_hash, base_name, base_logo, base_desc);
            self._init_erc20(erc20_code_hash, erc20_name, erc20_symbol, erc20_initial_supply, erc20_decimals);
            self._init_org(org_code_hash);
            self._init_vault(vault_code_hash);
            self._init_vote(vote_code_hash);
            self._init_github(github_code_hash);

            self.init = true;
            true
        }

        #[ink(message)]
        pub fn query_component_addrs(&self) -> DAOComponentAddrs {
            self.component_addrs
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u64) -> bool {
            let controller = self.env().caller();
            assert_eq!(self.components.erc20.is_some(), true);
            assert_eq!(controller == self.controller, true);
            let erc20 = self.components.erc20.as_mut().unwrap();
            erc20.transfer(to, value)
        }


        #[ink(message)]
        pub fn mint_token_by_owner(&mut self, to: AccountId, value: u64, ) -> bool {
            let controller = self.env().caller();
            assert_eq!(self.components.erc20.is_some(), true);
            assert_eq!(controller == self.controller, true);
            let erc20 = self.components.erc20.as_mut().unwrap();
            erc20.mint_token_by_owner(to, value)
        }

        #[ink(message)]
        pub fn destroy_token_by_owner(&mut self, from: AccountId, value: u64) -> bool {
            let controller = self.env().caller();
            assert_eq!(self.components.erc20.is_some(), true);
            assert_eq!(controller == self.controller, true);
            let erc20 = self.components.erc20.as_mut().unwrap();
            erc20.destroy_token_by_owner(from, value)
        }

        #[ink(message)]
        pub fn add_dao_moderator(&mut self, name: String, moderator: AccountId) -> bool {
            let controller = self.env().caller();
            assert_eq!(self.components.org.is_some(), true);
            assert_eq!(controller == self.controller, true);
            let org = self.components.org.as_mut().unwrap();
            org.add_dao_moderator(name, moderator)
        }

        #[ink(message)]
        pub fn remove_dao_moderator(&mut self, member: AccountId) -> bool {
            let controller = self.env().caller();
            assert_eq!(self.components.org.is_some(), true);
            assert_eq!(controller == self.controller, true);
            let org = self.components.org.as_mut().unwrap();
            org.remove_dao_moderator(member)
        }

        /// init base
        fn _init_base(&mut self, base_code_hash: Option<&Hash>,
                      base_name: String, base_logo: String, base_desc: String) -> bool {
            if base_code_hash.is_none() {
                return true;
            }
            let base_code_hash = base_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            // instance base
            let instance_params = Base::new()
                .endowment(total_balance / 4)
                .code_hash(base_code_hash)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Base` contract");
            let mut contract_instance: Base = ink_env::call::FromAccountId::from_account_id(contract_addr);
            contract_instance.init_base(base_name, base_logo, base_desc);

            self.components.base = Some(contract_instance);
            self.component_addrs.base_addr = Some(contract_addr);

            true
        }

        /// init erc20
        fn _init_erc20(&mut self, erc20_code_hash: Option<&Hash>, name: String, symbol: String, initial_supply: u64, decimals: u8) -> bool {
            if erc20_code_hash.is_none() {
                return true;
            }
            let erc20_code_hash = erc20_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            // instance erc20
            let erc20_instance_params = Erc20::new(name, symbol, initial_supply, decimals, Self::env().account_id())
                .endowment(total_balance / 4)
                .code_hash(erc20_code_hash)
                .params();
            let erc20_init_result = ink_env::instantiate_contract(&erc20_instance_params);
            let erc20_addr = erc20_init_result.expect("failed at instantiating the `Erc20` contract");
            let erc20_instance = ink_env::call::FromAccountId::from_account_id(erc20_addr);

            self.components.erc20 = Some(erc20_instance);
            self.component_addrs.erc20_addr = Some(erc20_addr);
            true
        }

        /// init org
        fn _init_org(&mut self, org_code_hash: Option<&Hash>) -> bool {
            if org_code_hash.is_none() {
                return true;
            }
            let org_code_hash = org_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            // instance org
            let org_instance_params = OrgManager::new(Self::env().account_id(), self.org_id)
                .endowment(total_balance / 4)
                .code_hash(org_code_hash)
                .params();
            let org_init_result = ink_env::instantiate_contract(&org_instance_params);
            let org_addr = org_init_result.expect("failed at instantiating the `Org` contract");
            let org_instance = ink_env::call::FromAccountId::from_account_id(org_addr);
            self.components.org = Some(org_instance);
            self.component_addrs.org_addr = Some(org_addr);
            true
        }

        /// init vault
        fn _init_vault(&mut self, vault_code_hash: Option<&Hash>) -> bool {
            if vault_code_hash.is_none() {
                return true;
            }
            let vault_code_hash = vault_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            // instance org
            let org_addr = self.component_addrs.org_addr.unwrap();
            let vault_instance_params = VaultManager::new(org_addr)
                .endowment(total_balance / 4)
                .code_hash(vault_code_hash)
                .params();
            let vault_init_result = ink_env::instantiate_contract(&vault_instance_params);
            let vault_addr = vault_init_result.expect("failed at instantiating the `Org` contract");
            let mut vault_instance: VaultManager = ink_env::call::FromAccountId::from_account_id(vault_addr);
            vault_instance.add_vault_token(self.component_addrs.erc20_addr.unwrap());
            self.components.vault = Some(vault_instance);
            self.component_addrs.vault_addr = Some(vault_addr);
            true
        }

        /// init vote
        fn _init_vote(&mut self, vote_code_hash: Option<&Hash>) -> bool {
            if vote_code_hash.is_none() {
                return true;
            }
            let vote_code_hash = vote_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            // instance org
            let vote_instance_params = VoteManager::new()
                .endowment(total_balance / 4)
                .code_hash(vote_code_hash)
                .params();
            let vote_init_result = ink_env::instantiate_contract(&vote_instance_params);
            let vote_addr = vote_init_result.expect("failed at instantiating the `Vote` contract");
            let vote_instance = ink_env::call::FromAccountId::from_account_id(vote_addr);
            self.components.vote = Some(vote_instance);
            self.component_addrs.vote_addr = Some(vote_addr);
            true
        }

        /// init github
        fn _init_github(&mut self, github_code_hash: Option<&Hash>) -> bool {
            if github_code_hash.is_none() {
                return true;
            }
            let github_code_hash = github_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            // instance org
            let github_instance_params = Github::new()
                .endowment(total_balance / 4)
                .code_hash(github_code_hash)
                .params();
            let github_init_result = ink_env::instantiate_contract(&github_instance_params);
            let github_addr = github_init_result.expect("failed at instantiating the `Github` contract");
            let github_instance = ink_env::call::FromAccountId::from_account_id(github_addr);
            self.components.github = Some(github_instance);
            self.component_addrs.github_addr = Some(github_addr);
            true
        }
    }
}
