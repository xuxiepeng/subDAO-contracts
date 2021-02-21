#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
pub use self::dao_manager::DAOManager;

#[ink::contract]
mod dao_manager {

    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},

    };
    use erc20::Erc20;
    use org::OrgManager;
    use vault::VaultManager;
    use vote_manager::VoteManager;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DAOManager {
        controller: AccountId,
        org_id: u64,
        erc20: Option<Erc20>,
        // TODO 保存每个子合约地址，用于查询
        org_manager: Option<OrgManager>,
        vault_manager: Option<VaultManager>,
        vote_manager: Option<VoteManager>,
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
                controller,
                org_id,
                erc20: None,
                org_manager: None,
                vault_manager: None,
                vote_manager: None,
            }
        }

        #[ink(message)]
        pub fn init_erc20(&mut self, erc20: Hash, initial_supply: u64, decimals: u8) -> bool {
            let total_balance = Self::env().balance();
            // instance erc20
            let erc20_instance_params = Erc20::new(initial_supply, decimals, Self::env().account_id())
                .endowment(total_balance / 4)
                .code_hash(erc20)
                .params();
            let erc20_init_result = ink_env::instantiate_contract(&erc20_instance_params);
            let erc20_addr = erc20_init_result.expect("failed at instantiating the `Erc20` contract");
            let erc20_instance = ink_env::call::FromAccountId::from_account_id(erc20_addr);
            // let mut erc20_instance = Erc20::new(initial_supply, decimals, Self::env().account_id())
            //     .endowment(total_balance / 4)
            //     .code_hash(erc20)
            //     .instantiate()
            //     .expect("failed at instantiating the `Erc20` contract");

            // TODO 增加脚本，修改metadata的名称，在编译完成后，根据wasm的名字修改
            self.erc20 = Some(erc20_instance);
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::Erc20,
                component_addr: erc20_addr,
            });
            true
        }

        #[ink(message)]
        pub fn init_org(&mut self, org_code_hash: Hash) -> bool {
            let total_balance = Self::env().balance();
            // instance org
            let org_instance_params = OrgManager::new(Self::env().account_id(), self.org_id)
                .endowment(total_balance / 4)
                .code_hash(org_code_hash)
                .params();
            let org_init_result = ink_env::instantiate_contract(&org_instance_params);
            let org_addr = org_init_result.expect("failed at instantiating the `Org` contract");
            let org_instance = ink_env::call::FromAccountId::from_account_id(org_addr);
            self.org_manager = Some(org_instance);
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::Org,
                component_addr: org_addr,
            });
            true
        }

        // TODO 合并其他简单的之合约实例化
        #[ink(message)]
        pub fn init_vault(&mut self, vault_code_hash: Hash) -> bool {
            let total_balance = Self::env().balance();
            // instance org
            let vault_instance_params = VaultManager::new(self.org_id)
                .endowment(total_balance / 4)
                .code_hash(vault_code_hash)
                .params();
            let vault_init_result = ink_env::instantiate_contract(&vault_instance_params);
            let vault_addr = vault_init_result.expect("failed at instantiating the `Org` contract");
            let vault_instance = ink_env::call::FromAccountId::from_account_id(vault_addr);
            self.org_manager = Some(vault_instance);
            self.env().emit_event(InstanceComponent {
                dao_addr: Self::env().account_id(),
                component_type: ComponentType::Vault,
                component_addr: vault_addr,
            });
            true
        }

        // TODO how to use vote
        // #[ink(message)]
        // pub fn init_vote_manager(&mut self, vote_code_hash: Hash) -> bool {
        //     let total_balance = Self::env().balance();
        //     // instance org
        //     let vault_instance_params = VaultManager::new(self.org_id)
        //         .endowment(total_balance / 4)
        //         .code_hash(vote_code_hash)
        //         .params();
        //     let vault_init_result = ink_env::instantiate_contract(&vault_instance_params);
        //     let vault_addr = vault_init_result.expect("failed at instantiating the `Org` contract");
        //     let vault_instance = ink_env::call::FromAccountId::from_account_id(vault_addr);
        //     self.org_manager = Some(vault_instance);
        //     self.env().emit_event(InstanceComponent {
        //         dao_addr: Self::env().account_id(),
        //         component_type: ComponentType::VoteManager,
        //         component_addr: vault_addr,
        //     });
        //     true
        // }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u64) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);
            let erc20 = self.erc20.as_mut().unwrap();
            erc20.transfer(to, value)
        }


        #[ink(message)]
        pub fn mint_token_by_owner(&mut self, to: AccountId, value: u64, ) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);
            let erc20 = self.erc20.as_mut().unwrap();
            erc20.mint_token_by_owner(to, value)
        }

        #[ink(message)]
        pub fn destroy_token_by_owner(&mut self, from: AccountId, value: u64) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.controller, true);
            let erc20 = self.erc20.as_mut().unwrap();
            erc20.destroy_token_by_owner(from, value)
        }

        // TODO 增加org的addDaoModerator和removeDaoModerator 委托调用
        // TODO 事件优化，可以看清楚初始化的组件和dao
        // TODO 实例化github，直接调用new()即可
        // TODO 实例化vote，参数让实例化dao的用户填入
    }
}
