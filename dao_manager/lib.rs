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
    use erc20::Erc20;
    use org::OrgManager;
    use vault::VaultManager;
    use vote_manager::VoteManager;
    use github::Github;

    /// DAO component instances
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponents {
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
        erc20_addr: Option<AccountId>,
        org_addr: Option<AccountId>,
        vault_addr: Option<AccountId>,
        vote_addr: Option<AccountId>,
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
        components: DAOComponents,
        component_addrs: DAOComponentAddrs,
    }

    #[derive(
    Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub enum ComponentType {
        Erc20,
        Org,
        Vault,
        VoteManager,
        Github,
    }

    #[ink(event)]
    pub struct InstanceComponent {
        #[ink(topic)]
        dao_addr: AccountId,
        #[ink(topic)]
        component_type: ComponentType,
        #[ink(topic)]
        component_addr: AccountId,
    }

    impl DAOManager {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(controller: AccountId, org_id: u64) -> Self {
            Self {
                init: false,
                controller,
                org_id,
                components: DAOComponents {
                    erc20: None,
                    org: None,
                    vault: None,
                    vote: None,
                    github: None,
                },
                component_addrs: DAOComponentAddrs {
                    erc20_addr: None,
                    org_addr: None,
                    vault_addr: None,
                    vote_addr: None,
                    github_addr: None,
                },
            }
        }

        #[ink(message)]
        pub fn init(&mut self, erc20_code_hash: Hash, erc20_initial_supply: u64, erc20_decimals: u8,
                    org_code_hash: Hash,
                    vault_code_hash: Hash,
                    vote_code_hash: Hash, vote_time: u64, vote_support_require_pct: u64, vote_min_require_num: u64,
                    github_code_hash: Hash) -> bool {
            assert_eq!(self.init, false);

            // init components
            self._init_erc20(erc20_code_hash, erc20_initial_supply, erc20_decimals);
            self._init_org(org_code_hash);
            self._init_vault(vault_code_hash);
            self._init_vote(vote_code_hash, vote_time, vote_support_require_pct, vote_min_require_num);
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
            assert_eq!(controller == self.controller, true);
            let erc20 = self.components.erc20.as_mut().unwrap();
            erc20.transfer(to, value)
        }


        #[ink(message)]
        pub fn mint_token_by_owner(&mut self, to: AccountId, value: u64, ) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);
            let erc20 = self.components.erc20.as_mut().unwrap();
            erc20.mint_token_by_owner(to, value)
        }

        #[ink(message)]
        pub fn destroy_token_by_owner(&mut self, from: AccountId, value: u64) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);
            let erc20 = self.components.erc20.as_mut().unwrap();
            erc20.destroy_token_by_owner(from, value)
        }

        #[ink(message)]
        pub fn add_dao_moderator(&mut self, name: String, moderator: AccountId) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);
            let org = self.components.org.as_mut().unwrap();
            org.add_dao_moderator(name, moderator)
        }

        #[ink(message)]
        pub fn remove_dao_moderator(&mut self, member: AccountId) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);
            let org = self.components.org.as_mut().unwrap();
            org.remove_dao_moderator(member)
        }

        /// init erc20
        fn _init_erc20(&mut self, erc20_code_hash: Hash, initial_supply: u64, decimals: u8) -> bool {
            let total_balance = Self::env().balance();
            // instance erc20
            let erc20_instance_params = Erc20::new(initial_supply, decimals, Self::env().account_id())
                .endowment(total_balance / 4)
                .code_hash(erc20_code_hash)
                .params();
            let erc20_init_result = ink_env::instantiate_contract(&erc20_instance_params);
            let erc20_addr = erc20_init_result.expect("failed at instantiating the `Erc20` contract");
            let erc20_instance = ink_env::call::FromAccountId::from_account_id(erc20_addr);

            // TODO 增加脚本，修改metadata的名称，在编译完成后，根据wasm的名字修改
            self.components.erc20 = Some(erc20_instance);
            self.component_addrs.erc20_addr = Some(erc20_addr);
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::Erc20,
                component_addr: erc20_addr,
            });
            true
        }

        /// init org
        fn _init_org(&mut self, org_code_hash: Hash) -> bool {
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
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::Org,
                component_addr: org_addr,
            });
            true
        }

        // TODO 合并其他简单的之合约实例化
        /// init vault
        fn _init_vault(&mut self, vault_code_hash: Hash) -> bool {
            let total_balance = Self::env().balance();
            // instance org
            let vault_instance_params = VaultManager::new(self.org_id)
                .endowment(total_balance / 4)
                .code_hash(vault_code_hash)
                .params();
            let vault_init_result = ink_env::instantiate_contract(&vault_instance_params);
            let vault_addr = vault_init_result.expect("failed at instantiating the `Org` contract");
            let vault_instance = ink_env::call::FromAccountId::from_account_id(vault_addr);
            self.components.vault = Some(vault_instance);
            self.component_addrs.vault_addr = Some(vault_addr);
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::Vault,
                component_addr: vault_addr,
            });
            true
        }

        /// init vote
        fn _init_vote(&mut self, vote_code_hash: Hash, vote_time: u64, support_require_pct: u64, min_require_num: u64) -> bool {
            let total_balance = Self::env().balance();
            // instance org
            let vote_instance_params = VoteManager::new(vote_time, support_require_pct, min_require_num)
                .endowment(total_balance / 4)
                .code_hash(vote_code_hash)
                .params();
            let vote_init_result = ink_env::instantiate_contract(&vote_instance_params);
            let vote_addr = vote_init_result.expect("failed at instantiating the `Vote` contract");
            let vote_instance = ink_env::call::FromAccountId::from_account_id(vote_addr);
            self.components.vote = Some(vote_instance);
            self.component_addrs.vote_addr = Some(vote_addr);
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::VoteManager,
                component_addr: vote_addr,
            });
            true
        }

        /// init github
        fn _init_github(&mut self, github_code_hash: Hash) -> bool {
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
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::Github,
                component_addr: github_addr,
            });
            true
        }
    }
}
