#![cfg_attr(not(feature = "std"), no_std)]

// extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod base {

    use std::string::String;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Base {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

    impl Base {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                name: String::default(),
                logo: String::default(),
                desc: String::default(),
                owner: Self::env().caller(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn set_name(&mut self, name: String) {
            self.name = name.to_string();
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn set_logo(&mut self, logo: String) {
            self.logo = String::from(logo);
        }

        #[ink(message)]
        pub fn get_logo(&self) -> String {
            self.logo
        }

        #[ink(message)]
        pub fn set_desc(&mut self, desc: String) {
            self.desc = String::from(desc);
        }

        #[ink(message)]
        pub fn get_desc(&self) -> String {
            self.desc
        }

        #[ink(message)]
        pub fn get_creator(&self) -> AccountId {
            self.owner
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test1() {
            let mut base = Base::default();
            base.set_name("hello".to_string());
            assert_eq!(base.get_name(), "hello");
            // assert_eq!("hello".to_string().cmp(&base.name), std::cmp::Ordering::Equal);
        }

        #[test]
        fn test2() {
            let eth_name = String::from("eth");
            assert_eq!(eth_name.clone(), "eth");
        }

        // #[test]
        // fn it_works() {
        //     let mut base = Base::new(false);
        //     assert_eq!(base.get(), false);
        //     base.flip();
        //     assert_eq!(base.get(), true);
        // }
    }
}
