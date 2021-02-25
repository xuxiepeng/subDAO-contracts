#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
pub use self::erc20::Erc20;

#[ink::contract]
mod erc20 {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[ink(storage)]
    pub struct Erc20 {
        total_supply: u64,
        decimals: u8,
        balances: StorageHashMap<AccountId, u64>,
        allowances: StorageHashMap<(AccountId, AccountId), u64>,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: u64,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: u64,
    }

    // TODO 增加name和symbol 用string
    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: u64, decimals: u8, controller: AccountId) -> Self {
            let mut balances = StorageHashMap::new();
            balances.insert(controller, initial_supply);
            let instance = Self {
                total_supply: initial_supply,
                decimals,
                balances,
                allowances: StorageHashMap::new(),
                owner: controller,
            };
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(controller),
                value: initial_supply,
            });
            instance
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u64 {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u64 {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u64 {
            self.allowance_of_or_zero(&owner, &spender)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u64) -> bool {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: u64) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            true
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u64,
        ) -> bool {
            let caller = self.env().caller();
            let allowance = self.allowance_of_or_zero(&from, &caller);
            if allowance < value {
                return false
            }
            self.allowances.insert((from, caller), allowance - value);
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn mint_token_by_owner(
            &mut self,
            to: AccountId,
            value: u64,
        ) -> bool {
            let caller = self.env().caller();
            assert_eq!(caller == self.owner, true);
            self._mint_token(to, value)
        }

        #[ink(message)]
        pub fn destroy_token_by_owner(
            &mut self,
            from: AccountId,
            value: u64,
        ) -> bool {
            let caller = self.env().caller();
            assert_eq!(caller == self.owner, true);
            self._destroy_token(from, value)
        }

        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u64,
        ) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> u64 {
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(
            &self,
            owner: &AccountId,
            spender: &AccountId,
        ) -> u64 {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }

        fn _mint_token(
            &mut self,
            to: AccountId,
            amount: u64,
        ) -> bool {
            let total_supply = self.total_supply();
            assert_eq!(total_supply + amount >= total_supply, true);
            let to_balance = self.balance_of_or_zero(&to);
            assert_eq!(to_balance + amount >= to_balance, true);
            self.total_supply += amount;
            self.balances.insert(to, to_balance + amount);
            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                value: amount,
            });
            true
        }

        fn _destroy_token(
            &mut self,
            from: AccountId,
            amount: u64,
        ) -> bool {
            let total_supply = self.total_supply();
            assert_eq!(total_supply - amount <= total_supply, true);
            let from_balance = self.balance_of_or_zero(&from);
            assert_eq!(from_balance - amount <= from_balance, true);
            self.total_supply -= amount;
            self.balances.insert(from, from_balance - amount);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: None,
                value: amount,
            });
            true
        }

    }
}
