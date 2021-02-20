// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
use alloc::vec::Vec;
pub use self::org::OrgManager;

#[ink::contract]
mod org {
    use alloc::string::String;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };


    #[ink(storage)]
    pub struct OrgManager {

        moderators: StorageHashMap<AccountId, String>,
        members: StorageHashMap<AccountId, String>,
        creator: AccountId,
        orgId:u64,
    }


    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        DaoModeratorExistAlready,
        DaoMemberExistAlready,
        DaoMemberNotExist,
        DaoModeratorNotExist,
    }

    #[ink(event)]
    pub struct addDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        orgId:u64,
    }

    #[ink(event)]
    pub struct removeDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        orgId:u64,
    }


    #[ink(event)]
    pub struct addDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        orgId:u64,
    }

    #[ink(event)]
    pub struct removeDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        orgId:u64,
    }


    impl OrgManager {

        #[ink(constructor)]
        pub fn new(_creator: AccountId,_orgId:u64) -> Self {
            Self {
                creator: _creator,
                orgId:_orgId,
                moderators: StorageHashMap::default(),
                members: StorageHashMap::default(),
            }
        }

        #[ink(message)]
        pub fn get_dao_creator(&self) -> AccountId {
            self.creator
        }

        #[ink(message)]
        pub fn get_orgid(&self) -> u64 {
            self.orgId
        }


        #[ink(message)]
        pub fn get_dao_moderator_list(&self) -> alloc::vec::Vec<AccountId> {
            self.moderators.keys();
            let mut v:alloc::vec::Vec<AccountId> = alloc::vec::Vec::new();
            for key in self.moderators.keys() {
                v.push(*key)
            }
            v

        }

        #[ink(message)]
        pub fn get_dao_members_list(&self) -> alloc::vec::Vec<AccountId> {
            self.members.keys();
            let mut v:alloc::vec::Vec<AccountId> = alloc::vec::Vec::new();
            for key in self.members.keys() {
                v.push(*key)
            }
            v
        }



        #[ink(message)]
        pub fn add_dao_moderator(&mut self,name:String,moderator: AccountId) -> bool  {
            let caller = self.env().caller();

            // 如果调用者不是组织创建者，即报错
            if &caller != & self.creator {
                return false;
            }


            match self.moderators.insert(moderator,name) {
                // 该成员已经存在，加入报错
                Some(_) => { false},
                None => {
                    let orgId = self.orgId;
                    self.env().emit_event(addDAOModeratorEvent{
                    moderator,
                    orgId,});
                    true
                }
            }
        }

        #[ink(message)]
        pub fn add_dao_member(&mut self,name:String,member: AccountId) -> bool {



            match self.members.insert(member,name) {
                // 该成员已经存在，加入报错
                Some(_) => { false},
                None => {
                    let orgId = self.orgId;
                    self.env().emit_event(addDAOMemberEvent{
                    member,
                    orgId,
                });
                    true
                }
            }

        }

        #[ink(message)]
        pub fn remove_dao_moderator(&mut self,member: AccountId) -> bool  {

            let caller = self.env().caller();

            // 如果调用者不是组织创建者，即报错
            if &caller != & self.creator {
                return false;
            }

            match self.moderators.take(&member) {
                // 该成员不存在，移除报错
                None => { false}
                Some(_) => {
                    let orgId = self.orgId;
                    self.env().emit_event(removeDAOModeratorEvent{
                    moderator:member,
                    orgId,
                });
                     true
                }
            }


        }

        #[ink(message)]
        pub fn remove_dao_member(&mut self, member: AccountId) -> bool  {

            match self.members.take(&member) {
                // 该成员不存在，移除报错
                None => { false}
                Some(_) => {
                    let orgId = self.orgId;
                    self.env().emit_event(removeDAOMemberEvent{
                    member:member,
                    orgId:orgId,
                });
                     true
                }
            }

        }


        #[ink(message)]
        //成员自我退出
        pub fn resign(&mut self,member: AccountId) -> bool  {


            //管理者角色退出
            if self.members.contains_key(&member) {
                self.members.take(&member);
                return true;
            };
            //普通用户角色的退出
            if self.moderators.contains_key(&member) {
                self.moderators.take(&member);
                return true;
            };
            return false;
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
        fn new_org_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);

            assert_eq!(org_manager.creator, accounts.alice);
            assert_eq!(org_manager.orgId, 1);
        }

        #[ink::test]
        fn add_member_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name,accounts.bob);
            let mut member = org_manager.get_dao_members_list()[0];
            assert_eq!(member, accounts.bob);

        }

        #[ink::test]
        fn add_moderator_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_moderator(bob_name,accounts.bob);
            let mut member = org_manager.get_dao_moderator_list()[0];
            assert_eq!(member, accounts.bob);

        }

        #[ink::test]
        fn remove_moderator_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_moderator(bob_name,accounts.bob);
            org_manager.remove_dao_moderator(accounts.bob);

            let mut members = org_manager.get_dao_moderator_list();
            assert_eq!(members.len(), 0);

        }


        #[ink::test]
        fn remove_members_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name,accounts.bob);
            org_manager.remove_dao_member(accounts.bob);
            let mut members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 0);
        }

        #[ink::test]
        fn resign_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name,accounts.bob);
            let eve_name = String::from("eve");
            org_manager.add_dao_member(eve_name,accounts.eve);
            let mut members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 2);
            org_manager.resign(accounts.bob);
            members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 1);
            org_manager.resign(accounts.eve);
            members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 0);
        }
    }
}
