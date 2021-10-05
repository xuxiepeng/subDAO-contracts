#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::base::Base;

#[ink::contract]
mod base {

    use alloc::string::String;
    use ink_storage::{
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DisplayBase {
        creator: AccountId,
        name: String,
        logo: String,
        desc: String,
    }
    
    #[ink(storage)]
    pub struct Base {
        creator: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

    impl Base {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                name: String::default(),
                logo: String::default(),
                desc: String::default(),
                creator: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn init_base(&mut self, name: String, logo: String, desc: String) {
            self.set_name(name);
            self.set_logo(logo);
            self.set_desc(desc);

            let caller = self.env().caller();
            self._set_creator(caller);
        }

        #[ink(message)]
        pub fn set_name(&mut self, name: String) {
            self.name = String::from(name);
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
            self.logo.clone()
        }

        #[ink(message)]
        pub fn set_desc(&mut self, desc: String) {
            self.desc = String::from(desc);
        }

        #[ink(message)]
        pub fn get_desc(&self) -> String {
            self.desc.clone()
        }

        pub fn _set_creator(&mut self, creator: AccountId) {

            let caller = self.env().caller();

            if self.creator == AccountId::default() || caller == self.creator {
                self.creator = creator;
            }
        }

        #[ink(message)]
        pub fn get_creator(&self) -> AccountId {
            self.creator
        }

        #[ink(message)]
        pub fn get_base(&self) -> DisplayBase {
            DisplayBase {
                creator: self.creator,
                name: self.name.clone(),
                logo: self.logo.clone(),
                desc: self.desc.clone(),
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn test_name() {
            let mut base = Base::default();

            base.set_name("SubDAO".to_string());

            let dbg_msg = format!("name is {}", base.get_name());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_name(), "SubDAO");
        }

        #[ink::test]
        fn test_logo() {
            let mut base = Base::default();

            base.set_logo("https://example.com/logo.jpg".to_string());

            let dbg_msg = format!("logo is {}", base.get_logo());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_logo(), "https://example.com/logo.jpg");
        }

        #[ink::test]
        fn test_desc() {
            let mut base = Base::default();

            base.set_desc("This is the one to rule all!".to_string());

            let dbg_msg = format!("name is {}", base.get_desc());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_desc(), "This is the one to rule all!");
        }

        #[ink::test]
        fn test_all() {

            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

            let mut base = Base::default();

            base.init_base("SubDAO".to_string(), "http://example.com/logo.jpg".to_string(), "This is the one to rule all!".to_string());

            let dbg_msg = format!("name is {}", base.get_name());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_name(), "SubDAO");
            assert_eq!(base.get_logo(), "http://example.com/logo.jpg");
            assert_eq!(base.get_desc(), "This is the one to rule all!");
            assert_eq!(base.get_creator(), accounts.alice);

            let dbg_msg2 = format!("name is {:?}", base.get_creator());
            ink_env::debug_println!("{}", &dbg_msg2 );
        }


        #[ink::test]
        fn test_meanless() {
            let dbg_msg = format!("name is eth");
            ink_env::debug_println!("{}", &dbg_msg );

            let eth_name = String::from("eth");
            assert_eq!(eth_name.clone(), "eth");
        }
    }
}
