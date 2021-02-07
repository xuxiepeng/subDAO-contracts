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

use ink_lang as ink;

#[ink::contract]
mod org {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };

    
    #[ink(storage)]
    pub struct OrgManager {

        moderators: StorageHashMap<Hash, AccountId>,
        members: StorageHashMap<Hash, AccountId>,
        creator: AccountId,
    }


    #[ink(event)]
    pub struct addDAOModeratorEvent {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        moderator: AccountId,
    }

    #[ink(event)]
    pub struct removeDAOModeratorEvent {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        moderator: AccountId,
    }


    #[ink(event)]
    pub struct addDAOMemberEvent {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        member: AccountId,
    }

    #[ink(event)]
    pub struct removeDAOMemberEvent {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        member: AccountId,
    }
    

    impl OrgManager {

        #[ink(constructor)]
        pub fn new(_creator: AccountId) -> Self {
            Self { 
                creator: _creator,
                moderators: StorageHashMap::default(),
                members: StorageHashMap::default(),
            }
        }

        #[ink(message)]
        pub fn get_dao_creator(&self) -> AccountId {
            self.creator
        }

        #[ink(message)]
        pub fn get_dao_moderator_list(&self) -> Keys {
            self.moderators.keys()
        }

        #[ink(message)]
        pub fn get_dao_members_list(&self) -> Keys {
            self.members.keys()
        }
        
        #[ink(message)]
        pub fn add_dao_moderator(&self,name:Hash,moderator: AccountId) -> Keys {
            self.moderators.insert(name,moderator)

            self.env().emit_event(addDAOModeratorEvent{
                name,
                moderator,
            });
        }

        #[ink(message)]
        pub fn add_dao_member(&self,name:Hash,member: AccountId) -> Keys {
            self.members.insert(name,member)

            self.env().emit_event(addDAOMemberEvent{
                name,
                member,
            });
        }

        #[ink(message)]
        pub fn remove_dao_moderator(&self,name:Hash) -> Keys {
            self.moderators.take(name)

            self.env().emit_event(removeDAOModeratorEvent{
                name,
                moderator,
            });
        }

        #[ink(message)]
        pub fn remove_dao_member(&self,name:Hash) -> Keys {
            self.members.take(name)

            self.env().emit_event(removeDAOMemberEvent{
                name,
                member,
            });
        }


        #[ink(message)]
        pub fn resign(&self,name:Hash) -> Keys {
            if self.members.contains_key(name) {
                self.members.take(name)
            }
            if self.moderators.contains_key(name) {
                self.moderators.take(name)
            }
        }
      
}
