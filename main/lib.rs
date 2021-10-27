#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod main {
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
    // use ink_prelude::string::String;
    use dao_manager::DAOManager;
    use org::OrgManager;
    use base::Base;
    use template_manager::TemplateManager;
    use template_manager::DAOTemplate;
    // const ONE_UNIT: u128 = 1_000_000_000_000;
    const TEMPLATE_INIT_BALANCE: u128 = 100 * 1000 * 1_000_000_000_000;
    const DAO_INIT_BALANCE: u128 = 1000 * 1000 * 1_000_000_000_000;

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]

    pub struct DAOInstance {
        id: u64,
        owner: AccountId,
        size: u64,
        name: String,
        logo: String,
        desc: String,
        dao_manager: DAOManager,
        dao_manager_addr: AccountId,
    }

    #[ink(storage)]
    pub struct Main {
        owner: AccountId,
        template_addr: Option<AccountId>,
        template: Option<TemplateManager>,
        instance_index: u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
        instance_map_by_owner: StorageHashMap<AccountId, Vec<u64>>,
    }

    #[ink(event)]
    pub struct InstanceDAO {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        dao_addr: AccountId,
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

    impl Main {
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            let instance = Self {
                owner: controller,
                template_addr: None,
                template: None,
                instance_index: 0,
                instance_map: StorageHashMap::new(),
                instance_map_by_owner: StorageHashMap::new(),
            };
            instance
        }
        #[ink(message)]
        pub fn  init (&mut self, template_code_hash: Hash, salt: Vec<u8>) -> bool
        {
            // let total_balance = Self::env().balance();
            // instance template_manager
            // let salt = version.to_le_bytes();
            let instance_params = TemplateManager::new(self.owner)
                .endowment(TEMPLATE_INIT_BALANCE)
                .code_hash(template_code_hash)
                .salt_bytes(&salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `TemplateManager` contract");
            let contract_instance = ink_env::call::FromAccountId::from_account_id(contract_addr);

            self.template = Some(contract_instance);
            self.template_addr = Some(contract_addr);
            true
        }

        #[ink(message)]
        pub fn add_template(&mut self, name: String, dao_manager_code_hash: Hash, components: BTreeMap<String, Hash>) -> bool {
            self.template.as_mut().unwrap().add_template(name, dao_manager_code_hash, components)
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
        pub fn list_templates(&self) -> Vec<DAOTemplate> {
            self.template.as_ref().unwrap().list_templates()
        }

        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
            self.template.as_ref().unwrap().query_template_by_index(index)
        }

        #[ink(message)]
        pub fn query_template_addr(&self) -> AccountId {
            self.template_addr.unwrap()
        }

        #[ink(message)]
        pub fn instance_by_template(&mut self, index: u64, controller: AccountId, salt: Vec<u8>) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            // let total_balance = Self::env().balance();
            // assert_eq!(total_balance >= 20, true);

            // instance dao_manager
            let template = self.query_template_by_index(index);
            let dao_manager_code_hash = template.dao_manager_code_hash;
            // let salt = version.to_le_bytes();
            let dao_instance_params = DAOManager::new(controller, self.instance_index)
                .endowment(DAO_INIT_BALANCE)
                .code_hash(dao_manager_code_hash)
                .salt_bytes(salt)
                .params();
            let dao_init_result = ink_env::instantiate_contract(&dao_instance_params);
            let dao_addr = dao_init_result.expect("failed at instantiating the `DAO Instance` contract");
            let mut dao_instance: DAOManager = ink_env::call::FromAccountId::from_account_id(dao_addr);
            dao_instance.set_template(template);
            self.env().emit_event(InstanceDAO {
                index: self.instance_index,
                owner: Some(controller),
                dao_addr: dao_addr,
            });

            let id_list = self.instance_map_by_owner.entry(controller.clone()).or_insert(Vec::new());
            id_list.push(self.instance_index);
            self.instance_map.insert(self.instance_index, DAOInstance {
                id: self.instance_index,
                owner: controller,
                size: 0,
                name: String::from(""),
                logo: String::from(""),
                desc: String::from(""),
                dao_manager: dao_instance,
                dao_manager_addr: dao_addr,
            });
            self.instance_index += 1;
            true
        }

        #[ink(message)]
        pub fn list_dao_instances(&mut self, page:u64, size:u64) -> PageResult<DAOInstance> {
            let total = self.instance_map.len() as u64;
            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut total_keys_vec = Vec::new();
            for elem in self.instance_map.keys() {
                total_keys_vec.push(elem);
            }

            let mut dao_vec = Vec::new();
            for i in start..end {
                let key = total_keys_vec.get(i as usize);
                if let Some(s) = key {
                    dao_vec.push( Main::fill_dao_details(self.instance_map.get(s).unwrap().clone()));
                }
            }

            return PageResult{
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: dao_vec,
            }
        }

        #[ink(message)]
        pub fn list_dao_info(&mut self, dao_addr:AccountId) -> DAOInstance {
            let dao_instance: DAOManager = ink_env::call::FromAccountId::from_account_id(dao_addr);
            let dao = Main::fill_dao_details(DAOInstance {
                id: self.instance_index,
                owner: Default::default(),
                size: 0,
                name: String::from(""),
                logo: String::from(""),
                desc: String::from(""),
                dao_manager: dao_instance,
                dao_manager_addr: dao_addr,
            });
            dao
        }

        fn fill_dao_details(mut dao: DAOInstance) -> DAOInstance {
            let org_addr_op = dao.dao_manager.query_component_addrs().org_addr;
            if org_addr_op.is_none() {
                return dao
            }
            let org_addr: AccountId = org_addr_op.unwrap();
            let org_instance: OrgManager = ink_env::call::FromAccountId::from_account_id(org_addr);
            dao.owner = org_instance.get_dao_owner();
            dao.size = org_instance.get_dao_size();

            let base_addr_op = dao.dao_manager.query_component_addrs().base_addr;
            if base_addr_op.is_none() {
                return dao
            }
            let base_addr: AccountId = base_addr_op.unwrap();
            let base_instance: Base = ink_env::call::FromAccountId::from_account_id(base_addr);
            dao.name = base_instance.get_name();
            dao.logo = base_instance.get_logo();
            dao.desc = base_instance.get_desc();
            
            dao
        }
        #[ink(message)]
        pub fn list_last_dao_instances_by_owner(&mut self, owner: AccountId) -> DAOInstance {
            
            let index = self.instance_map_by_owner.get(&owner).unwrap().last().unwrap();
            
            Main::fill_dao_details(self.instance_map.get(index).unwrap().clone())

        }


        #[ink(message)]
        pub fn list_dao_instances_by_owner(&mut self, owner: AccountId, page:u64, size:u64) -> PageResult<DAOInstance> {
            
            let id_list_op = self.instance_map_by_owner.get(&owner);

            let mut dao_vec = Vec::new();

            if id_list_op.is_some() {

                let id_list = id_list_op.unwrap();

                let total = id_list.len() as u64;
                let (start, end, pages) = self.cal_pages(page, size, total);

                for i in start..end {
                    let key = id_list.get(i as usize);
                    if let Some(s) = key {
                        let dao: DAOInstance = Main::fill_dao_details(self.instance_map.get(s).unwrap().clone());
                        dao_vec.push(dao);
                    }
                }

                return PageResult{
                    success: true,
                    err: String::from("success"),
                    total,
                    pages,
                    page: page,
                    size: size,
                    data: dao_vec,
                }
            }else{
                return PageResult{
                    success: true,
                    err: String::from("success"),
                    total: 0,
                    pages: 0,
                    page: page,
                    size: size,
                    data: dao_vec,
                }
            }    
        }

        #[ink(message)]
        pub fn list_dao_instances_by_account(&mut self, user: AccountId, page:u64, size:u64) -> PageResult<DAOInstance> {

            let mut total_keys_vec = Vec::new();
            for elem in self.instance_map.keys() {
                if let Some(dao) = self.instance_map.get(elem) {

                    let org_addr_op = dao.dao_manager.query_component_addrs().org_addr;
                    if org_addr_op.is_none() {
                        continue;
                    }
                    let org_addr: AccountId = org_addr_op.unwrap();
                    let org_instance: OrgManager = ink_env::call::FromAccountId::from_account_id(org_addr);
                    let (is_member, is_moderator, is_owner) = org_instance.check_role_by_account(user);
                    if is_member || is_moderator || is_owner {
                        total_keys_vec.push(elem);
                    }
                }    
            }

            let total = total_keys_vec.len() as u64;
            let (start, end, pages) = self.cal_pages(page, size, total);

            let mut dao_vec = Vec::new();
            for i in start..end {
                let key = total_keys_vec.get(i as usize);
                if let Some(s) = key {
                    let dao: DAOInstance = Main::fill_dao_details(self.instance_map.get(s).unwrap().clone());
                    dao_vec.push(dao);
                }
            }

            return PageResult{
                success: true,
                err: String::from("success"),
                total,
                pages,
                page: page,
                size: size,
                data: dao_vec,
            }
        }
    }
}
