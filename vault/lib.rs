#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
//use ink_prelude::vec::Vec;
pub use self::vault::VaultManager;

#[ink::contract]
mod vault {

    use alloc::string::String;

    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    use auth::Auth;
    use erc20::Erc20;
    use org::OrgManager;

    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        scale::Encode,
        scale::Decode,
        SpreadLayout,
        PackedLayout,
        Default,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Transfer {
        transfer_id: u64,
        transfer_direction: u64, // 1: out 2 : in
        token_name: String,
        from_address: AccountId,
        to_address: AccountId,
        value: u64,
        transfer_time: u64,
    }

    // Token info for query purpose.
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        scale::Encode,
        scale::Decode,
        SpreadLayout,
        PackedLayout,
        Default,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct TokenInfo {
        erc20: AccountId,
        symbol: String,
        name: String,
        balance: u64,
    }

    #[ink(storage)]
    pub struct VaultManager {
        tokens: StorageHashMap<AccountId, AccountId>,
        visible_tokens: StorageHashMap<AccountId, AccountId>,
        transfer_history: StorageHashMap<u64, Transfer>,
        org_contract_address: AccountId,
        vault_contract_address: AccountId,
        auth_contract_address: AccountId,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        InvalidTransferRecord,
    }

    #[ink(event)]
    pub struct AddVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,
    }

    #[ink(event)]
    pub struct RemoveVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,
    }

    #[ink(event)]
    pub struct GetTokenBalanceEvent {
        #[ink(topic)]
        token_address: AccountId,

        #[ink(topic)]
        balance: u64,
    }

    #[ink(event)]
    pub struct DepositTokenEvent {
        #[ink(topic)]
        token_name: String,
        #[ink(topic)]
        from_address: AccountId,

        #[ink(topic)]
        value: u64,
    }

    #[ink(event)]
    pub struct WithdrawTokenEvent {
        #[ink(topic)]
        token_name: String,

        #[ink(topic)]
        to_address: AccountId,

        #[ink(topic)]
        value: u64,
    }

    #[derive(Debug, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo,))]
    pub struct PageResult<T> {
        pub success: bool,
        pub err: String,
        pub total: u64,
        pub pages: u64,
        pub page: u64,
        pub size: u64,
        pub data: ink_prelude::vec::Vec<T>,
    }

    impl VaultManager {
        fn cal_pages(&self, page: u64, size: u64, total: u64) -> (u64, u64, u64) {
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

        #[ink(constructor)]
        pub fn new(org_contract_address: AccountId, auth_contract_address: AccountId) -> Self {
            let vault_contract_address = Self::env().account_id();

            Self {
                org_contract_address: org_contract_address,
                auth_contract_address: auth_contract_address,
                tokens: StorageHashMap::default(),
                visible_tokens: StorageHashMap::default(),
                transfer_history: StorageHashMap::default(),
                vault_contract_address: vault_contract_address,
            }
        }

        pub fn get_erc20_by_address(&self, address: AccountId) -> Erc20 {
            let erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(address);
            erc20_instance
        }

        pub fn get_auth_by_address(&self, address: AccountId) -> Auth {
            let auth_instance: Auth = ink_env::call::FromAccountId::from_account_id(address);
            auth_instance
        }

        pub fn get_orgmanager_by_address(&self, address: AccountId) -> OrgManager {
            let org_instance: OrgManager = ink_env::call::FromAccountId::from_account_id(address);
            org_instance
        }

        #[ink(message)]
        pub fn add_vault_token(&mut self, erc_20_address: AccountId) -> bool {
            let _caller = self.env().caller();

            let _auth = self.get_auth_by_address(self.auth_contract_address);

            // let is_permission = auth.has_permission(caller,String::from("vault"),String::from("add_vault_token"));
            let is_permission = true;

            if is_permission == false {
                return false;
            }

            match self
                .tokens
                .insert(erc_20_address, self.vault_contract_address)
            {
                Some(_) => false,
                None => {
                    self.visible_tokens
                        .insert(erc_20_address, self.vault_contract_address);

                    self.env().emit_event(AddVaultTokenEvent {
                        token_address: erc_20_address,
                    });
                    true
                }
            }
        }

        #[ink(message)]
        pub fn remove_vault_token(&mut self, erc_20_address: AccountId) -> bool {
            let _caller = self.env().caller();

            let _auth = self.get_auth_by_address(self.auth_contract_address);

            //let is_permission = auth.has_permission(caller,String::from("vault"),String::from("remove_vault_token"));
            let is_permission = true;

            if is_permission == false {
                return false;
            }

            match self.visible_tokens.take(&erc_20_address) {
                None => false,
                Some(_) => {
                    self.env().emit_event(RemoveVaultTokenEvent {
                        token_address: erc_20_address,
                    });
                    true
                }
            }
        }

        #[ink(message)]
        pub fn get_token_list(&self, page: u64, size: u64) -> PageResult<AccountId> {
            // the last one is native token.
            let total = self.visible_tokens.len() as u64 + 1;

            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut data_vec: ink_prelude::vec::Vec<AccountId> = ink_prelude::vec::Vec::new();

            for i in start..end {
                // add native token
                if i == total - 1 {
                    data_vec.push(AccountId::from([0xee; 32]));
                } else {
                    let key = self.visible_tokens.keys().nth(i as usize).unwrap().clone();
                    data_vec.push(key);
                }
            }

            return PageResult {
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: data_vec,
            };
        }

        #[ink(message)]
        pub fn get_balance_of(&self, erc_20_address: AccountId) -> u64 {
            if erc_20_address == AccountId::from([0xee; 32]) {
                let value = self.env().balance() as u64;
                return value;
            }
            if self.tokens.contains_key(&erc_20_address) {
                // let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let erc_20 = self.get_erc20_by_address(erc_20_address);
                //let token_name = (&erc_20).name();
                let balanceof = erc_20.balance_of(self.vault_contract_address);

                self.env().emit_event(GetTokenBalanceEvent {
                    token_address: erc_20_address,
                    balance: balanceof,
                });

                balanceof
            } else {
                0
            }
        }

        #[ink(message)]
        pub fn get_balance(&self, page: u64, size: u64) -> PageResult<TokenInfo> {
            // the last one is native token.
            let total = self.visible_tokens.len() as u64 + 1;

            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut data_vec: ink_prelude::vec::Vec<TokenInfo> = ink_prelude::vec::Vec::new();

            for i in start..end {
                // add native token
                if i == total - 1 {
                    let value = self.env().balance() as u64;
                    data_vec.push(TokenInfo {
                        erc20: AccountId::from([0xee; 32]),
                        symbol: String::from("GOV"),
                        name: String::from("SubDAO"),
                        balance: value,
                    });
                } else {
                    let address = self.visible_tokens.keys().nth(i as usize).unwrap().clone();
                    let erc20_instance: Erc20 =
                        ink_env::call::FromAccountId::from_account_id(address);
                    data_vec.push(TokenInfo {
                        erc20: address,
                        symbol: erc20_instance.symbol(),
                        name: erc20_instance.name(),
                        balance: erc20_instance.balance_of(self.vault_contract_address),
                    });
                }
            }

            return PageResult {
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: data_vec,
            };
        }

        #[ink(message)]
        pub fn deposit(
            &mut self,
            erc_20_address: AccountId,
            from_address: AccountId,
            value: u64,
        ) -> bool {
            if erc_20_address == AccountId::from([0xee; 32]) {
                return self.deposit_native_token();
            }
            let to_address = self.vault_contract_address;

            if self.tokens.contains_key(&erc_20_address) {
                // let  balanceof =  self.get_balance_of(erc_20_address);

                //let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);

                let token_name = (&erc_20).name();

                let transfer_result = erc_20.transfer_from(from_address, to_address, value);

                if transfer_result == false {
                    return false;
                }

                let transfer_id: u64 = (self.transfer_history.len() + 1).into();

                let transfer_time: u64 = self.env().block_timestamp();

                self.transfer_history.insert(
                    transfer_id,
                    Transfer {
                        transfer_direction: 2, // 1: out 2: in
                        token_name: token_name.clone(),
                        transfer_id: transfer_id,
                        from_address: from_address,
                        to_address: to_address,
                        value,
                        transfer_time,
                    },
                );

                self.env().emit_event(DepositTokenEvent {
                    token_name: token_name.clone(),
                    from_address: from_address,
                    value: value,
                });
                true
            } else {
                false
            }
        }

        pub fn deposit_native_token(&mut self) -> bool {
            let from_address = self.env().caller();
            let value = self.env().transferred_balance();
            assert!(value > 0, "value is 0");
            let to_address = self.vault_contract_address;

            let transfer_id: u64 = (self.transfer_history.len() + 1).into();
            let transfer_time: u64 = self.env().block_timestamp();

            let value2 = value as u64;
            self.transfer_history.insert(
                transfer_id,
                Transfer {
                    transfer_direction: 2, // 1: out 2: in
                    token_name: String::from("SubDAO"),
                    transfer_id: transfer_id,
                    from_address: from_address,
                    to_address: to_address,
                    value: value2,
                    transfer_time,
                },
            );

            self.env().emit_event(DepositTokenEvent {
                token_name: String::from("SubDAO"),
                from_address: from_address,
                value: value2,
            });
            true
        }

        #[ink(message)]
        pub fn withdraw(
            &mut self,
            erc_20_address: AccountId,
            to_address: AccountId,
            value: u64,
        ) -> bool {
            if erc_20_address == AccountId::from([0xee; 32]) {
                return self.withdraw_native_token(to_address, value.into());
            }
            let from_address = self.vault_contract_address;

            if self.visible_tokens.contains_key(&erc_20_address) {
                let _caller = self.env().caller();

                let _auth = self.get_auth_by_address(self.auth_contract_address);

                // let is_permission = auth.has_permission(caller,String::from("vault"),String::from("withdraw"));
                let is_permission = true;

                if is_permission == false {
                    return false;
                }

                // let  balanceof =  self.get_balance_of(erc_20_address);

                //let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);

                let token_name = (&erc_20).name();

                //erc_20.transfer_from(from_address,to_address, value);

                let transfer_result = erc_20.transfer(to_address, value);

                if transfer_result == false {
                    return false;
                }

                let transfer_id: u64 = (self.transfer_history.len() + 1).into();

                let transfer_time: u64 = self.env().block_timestamp();

                self.transfer_history.insert(
                    transfer_id,
                    Transfer {
                        transfer_direction: 1, // 1: out 2: in
                        token_name: token_name.clone(),
                        transfer_id: transfer_id,
                        from_address: from_address,
                        to_address: to_address,
                        value: value,
                        transfer_time: transfer_time,
                    },
                );

                self.env().emit_event(WithdrawTokenEvent {
                    token_name: token_name.clone(),
                    to_address: to_address,
                    value: value,
                });

                true
            } else {
                false
            }
        }

        pub fn withdraw_native_token(&mut self, to_address: AccountId, value: u128) -> bool {
            let from_address = self.vault_contract_address;
            let balance = self.env().balance();
            assert!(balance >= value, "balance is not enough");

            let _caller = self.env().caller();

            let _auth = self.get_auth_by_address(self.auth_contract_address);

            // let is_permission = auth.has_permission(caller,String::from("vault"),String::from("withdraw"));
            let is_permission = true;

            if is_permission == false {
                return false;
            }

            match self.env().transfer(to_address, value.into()) {
                Err(_) => panic!("transfer failed!"),
                Ok(_) => {}
            }

            let value2 = value as u64;
            let transfer_id: u64 = (self.transfer_history.len() + 1).into();

            let transfer_time: u64 = self.env().block_timestamp();

            self.transfer_history.insert(
                transfer_id,
                Transfer {
                    transfer_direction: 1, // 1: out 2: in
                    token_name: String::from("SubDAO"),
                    transfer_id: transfer_id,
                    from_address: from_address,
                    to_address: to_address,
                    value: value2,
                    transfer_time: transfer_time,
                },
            );

            self.env().emit_event(WithdrawTokenEvent {
                token_name: String::from("SubDAO"),
                to_address: to_address,
                value: value2,
            });

            true
        }

        #[ink(message)]
        pub fn get_transfer_history(&self, page: u64, size: u64) -> PageResult<Transfer> {
            let total = self.transfer_history.len() as u64;

            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut data_vec: ink_prelude::vec::Vec<Transfer> = ink_prelude::vec::Vec::new();

            for i in start..end {
                let value = self
                    .transfer_history
                    .values()
                    .rev()
                    .nth(i as usize)
                    .unwrap();
                data_vec.push(value.clone());
            }

            return PageResult {
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: data_vec,
            };
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        // use ink_env::{
        //     call,
        //     test,
        // };
        use ink_lang as ink;

        #[ink::test]
        fn add_token_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new(accounts.alice, accounts.alice);
            vault_manager.add_vault_token(accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
        }

        #[ink::test]
        fn remove_token_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new(accounts.alice, accounts.alice);
            vault_manager.add_vault_token(accounts.bob);
            vault_manager.remove_vault_token(accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
            assert_eq!(vault_manager.visible_tokens.len(), 0);
        }

        #[ink::test]
        fn get_token_list_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new(accounts.alice, accounts.alice);
            vault_manager.add_vault_token(accounts.bob);
            vault_manager.add_vault_token(accounts.alice);
            assert_eq!(vault_manager.get_token_list().len(), 2);
        }

        #[ink::test]
        fn get_balance_of_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new(accounts.alice, accounts.alice);
            vault_manager.add_vault_token(accounts.bob);
            assert_eq!(vault_manager.get_balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn deposit_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new(accounts.alice, accounts.alice);
            vault_manager.add_vault_token(accounts.bob);
            vault_manager.deposit(accounts.bob, accounts.alice, 100);
            assert_eq!(vault_manager.get_balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn withdraw_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new(accounts.alice, accounts.alice);
            vault_manager.add_vault_token(accounts.bob);
            vault_manager.deposit(accounts.bob, accounts.eve, 1000);
            vault_manager.withdraw(accounts.bob, accounts.alice, 100);
            assert_eq!(vault_manager.get_balance_of(accounts.bob), 900);
        }

        #[ink::test]
        fn transfer_history_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new(accounts.alice, accounts.alice);
            vault_manager.add_vault_token(accounts.bob);
            vault_manager.deposit(accounts.bob, accounts.eve, 1000);
            vault_manager.withdraw(accounts.bob, accounts.alice, 100);
            assert_eq!(vault_manager.get_transfer_history().len(), 2);
        }
    }
}
