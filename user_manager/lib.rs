#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

pub use self::user_manager::UserManager;

#[ink::contract]
mod user_manager {

    #[cfg(not(feature = "ink-as-dependency"))]
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct UserManager {
        /// Stores a single `bool` value on the storage.
        instance_map_by_user: StorageHashMap<AccountId, Vec<u64>>,
    }

    impl UserManager {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { instance_map_by_user: StorageHashMap::new() }
        }

        #[ink(message)]
        pub fn update(&mut self, user: AccountId, instance_index: u64) {
            let id_list = self.instance_map_by_user.entry(user.clone()).or_insert(Vec::new());
            let mut find = false;
            for x in id_list.iter() {
                if *x == instance_index {
                    find = true;
                }
            }

            if !find {
                id_list.push(instance_index);
            }
        }

        #[ink(message)]
        pub fn get(&self, user: AccountId ) -> Vec<u64> {
            let id_list = self.instance_map_by_user.get(&user);
            if let Some(s) = id_list {
                s.to_vec()
            }else{
                Vec::new()
            }

        }

        #[ink(message)]
        pub fn remove(&mut self, user: AccountId, instance_index: u64  ) {
            let id_list = self.instance_map_by_user.get(&user);
            let mut id_vec = Vec::new();
            if let Some(s) = id_list {
                if s.len() > 0 {
                    for x in s {
                        if *x != instance_index {
                            id_vec.push(x.clone())
                        } 
                    }
                    self.instance_map_by_user.insert(user, id_vec);
                }
            }
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{
            call,
            test,
        };
        use ink_lang as ink;

        #[ink::test]
        fn new_user_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            let mut user_manager = UserManager::new();
            user_manager.update(accounts.alice, 1);
            println!("Vector: {:?}", user_manager.get(accounts.alice));
            assert_eq!(user_manager.get(accounts.alice).len(), 1);
            assert_eq!(user_manager.get(accounts.alice)[0], 1);

            user_manager.update(accounts.alice, 1);
            user_manager.update(accounts.alice, 2);
            user_manager.update(accounts.alice, 3);
            println!("Vector: {:?}", user_manager.get(accounts.alice));

            assert_eq!(user_manager.get(accounts.alice).len(), 3);

            user_manager.remove(accounts.alice, 2);
            println!("Vector: {:?}", user_manager.get(accounts.alice));
            assert_eq!(user_manager.get(accounts.alice).len(), 2);

        }

    }

}
