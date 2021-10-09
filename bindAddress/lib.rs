#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::bind_address_manager::BindingManager;
pub use self::bind_address_manager::PageResult;


#[ink::contract]
mod bind_address_manager {

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };
    #[ink(storage)]
    pub struct BindingManager {
        // contract creator
        owner: AccountId,
        // index
        index: u64,
        // address map
        address_map: StorageHashMap<AccountId, String>,
    }

    #[ink(event)]
    pub struct BindAddressEvt {
        #[ink(topic)]
        address: String,
        #[ink(topic)]
        creator: AccountId,
    }

    #[derive(Debug, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct AddressInfo {
        pub address: AccountId, 
        pub eth: String
    }

    #[derive(Debug, scale::Encode, scale::Decode, Clone, )]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, )
    )]
    pub struct PageResult<T> {
        pub success: bool,
        pub err: String,
        pub total: u64,
        pub pages: u64,
        pub page: u64,
        pub size: u64,
        pub data: Vec<T>,
    }

    impl BindingManager {

        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                index: 0,
                address_map: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner.clone()
        }


        #[ink(message)]
        pub fn bind(&mut self, eth_address: String) -> bool {
            
            let caller = self.env().caller();
            if self.address_map.contains_key(&caller) {
                return false;
            }

            self.address_map.insert(caller, eth_address);
            true
        }

        fn cal_pages(&self, page:u64, size:u64, total: u64) -> (u64, u64, u64) {
            let start = page * size;
            let mut end = start + size;
            if end > total {
                end = total
            }
            assert!(size <= 0 || start >= total || start < end, "wrong params");
            let mut pages = total / size;
            if total % size > 0 {
                pages += 1;
            }
            (start, end, pages)
        }

        #[ink(message)]
        pub fn list_addresses(&self, page:u64, size:u64) -> PageResult<AddressInfo> {
            let total = self.address_map.len() as u64;
            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut total_address_vec = Vec::new();
            for elem in self.address_map.keys() {
                total_address_vec.push(AddressInfo{address:elem.clone(), eth: self.address_map.get(&elem).unwrap().clone()});
            }

            let mut address_vec = Vec::new();
            for i in start..end {
                let opt = total_address_vec.get(i as usize);
                if let Some(s) = opt {
                    address_vec.push(s.clone());
                }
            }
            return PageResult{
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: address_vec,
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
        fn test_bind_address() {
           
        }

        #[ink::test]
        fn test_get_list() {
           
        }

    }
}
