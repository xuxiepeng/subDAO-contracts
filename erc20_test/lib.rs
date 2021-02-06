#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
pub use self::erc20_test::Erc20Test;

#[ink::contract]
mod erc20_test {

    use erc20::Erc20;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Erc20Test {
        erc20: Erc20,
    }

    impl Erc20Test {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(erc20: Erc20,) -> Self {
            Self {
                erc20,
            }
        }

        #[ink(message)]
        pub fn transfer_in_erc20(&mut self, to: AccountId, value: u64) -> bool {
            self.erc20.transfer(to, value)
        }
    }
}
