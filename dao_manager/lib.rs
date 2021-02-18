#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
pub use self::dao_manager::DAOManager;

#[ink::contract]
mod dao_manager {

    use erc20::Erc20;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DAOManager {
        controller: AccountId,
        erc20: Option<Erc20>,
    }

    #[ink(event)]
    pub struct InstanceComponent {
        #[ink(topic)]
        dao_addr: Option<AccountId>,
        #[ink(topic)]
        component_addr: Option<AccountId>,
    }

    impl DAOManager {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            Self {
                controller,
                erc20: None,
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
            let erc20_instance_res = ink_env::instantiate_contract(&erc20_instance_params);
            let erc20_account_id = erc20_instance_res.expect("failed at instantiating the `Erc20` contract");
            let erc20_instance = ink_env::call::FromAccountId::from_account_id(erc20_account_id);
            // let mut erc20_instance = Erc20::new(initial_supply, decimals, Self::env().account_id())
            //     .endowment(total_balance / 4)
            //     .code_hash(erc20)
            //     .instantiate()
            //     .expect("failed at instantiating the `Erc20` contract");

            self.erc20 = Some(erc20_instance);
            self.env().emit_event(InstanceComponent {
                dao_addr: Some(Self::env().account_id()),
                // component_addr: Some(erc20_instance.to_account_id()),
                component_addr: Some(erc20_account_id),
            });
            true
        }

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
    }
}
