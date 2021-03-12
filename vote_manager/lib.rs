#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::vote_manager::VoteManager;

#[ink::contract]
mod vote_manager {

    use alloc::format;
    use alloc::vec;
    use alloc::vec::Vec;
    use alloc::string::String;

    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    type VoteId = u64;
    type ChoiceId = u32;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    enum VoterState {
        Absent,
        Yea,
        Nay,
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Clone,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Choice {
        choice_id: ChoiceId,
        content: String,
        yea: u64,
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Vote {
        vote_id: VoteId,
        executed: bool,
        title: String,
        desc: String,
        start_date: u64,
        vote_time: u64,
        support_require_num: u64,
        min_require_num: u64,
        support_num: u64,
        choice_index_lo: u32,
        choice_index_ho: u32,
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct DisplayVote {
        vote_id: VoteId,
        executed: bool,
        title: String,
        desc: String,
        start_date: u64,
        vote_time: u64,
        support_require_num: u64,
        min_require_num: u64,
        support_num: u64,
        choices: String,
    }


    #[ink(storage)]
    pub struct VoteManager {
        votes_length: u64,
        votes: StorageHashMap<VoteId, Vote>,
        voters: StorageHashMap<(VoteId, AccountId), ChoiceId>,
        choices: StorageVec<Choice>,
        choices_num: u32,
    }

    #[ink(event)]
    pub struct StartVote {
        #[ink(topic)]
        vote_id: VoteId,

        #[ink(topic)]
        creator: AccountId,
    }

    #[ink(event)]
    pub struct CastVote {
        #[ink(topic)]
        vote_id: VoteId,

        #[ink(topic)]
        voter: AccountId,

        support_choice: ChoiceId,
    }

    #[ink(event)]
    pub struct ExecuteVote {
        #[ink(topic)]
        vote_id: VoteId,
    }

    impl VoteManager {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                votes_length: 0,
                votes: StorageHashMap::default(),
                voters: StorageHashMap::default(),
                choices: StorageVec::default(),
                choices_num: 0,
            }
        }

        #[ink(message)]
        pub fn new_vote(&mut self, title: String, desc: String, vote_time: u64, support_require_num: u64, min_require_num: u64, choices: String) -> u64 {
            let vote_id = self.votes_length.clone();
            self.votes_length += 1;
            let start_date: u64 = self.env().block_timestamp();
            let vec: Vec<&str> = choices.split("|").collect();
            let vote = Vote{
                vote_id: vote_id,
                executed: false,
                title,
                desc,
                start_date: start_date,
                vote_time,
                support_require_num,
                min_require_num,
                support_num: 0,
                choice_index_lo: self.choices_num,
                choice_index_ho: self.choices_num + vec.len() as u32,
            };
            self.choices_num += vec.len() as u32;
            let mut index = 0;
            for choice_content in vec.iter() {
                self.choices.push(Choice{
                    choice_id: index,
                    content: String::from(*choice_content),
                    yea: 0,
                });
                index += 1;
            }
            self.votes.insert(vote_id, vote);
            self.env().emit_event(StartVote{
                vote_id,
                creator: self.env().caller(),
            });
            vote_id
        }

        #[ink(message)]
        pub fn execute(&mut self, vote_id: VoteId) {
            assert!(self.vote_exists(vote_id));
            let vote = self.votes.get(&vote_id).unwrap(); 
            if self.can_execute(&vote) {
                let mut vote = self.votes.get_mut(&vote_id).unwrap(); 
                vote.executed = true;
                self.env().emit_event(ExecuteVote{
                    vote_id,
                });
            }
            // true
        }

        #[ink(message)]
        pub fn vote(&mut self, vote_id: VoteId, support_choice: u32, voter: AccountId) -> bool {
            if !self.vote_exists(vote_id) {
                return false;
            }
            if let Some(vote) = self.votes.get_mut(&vote_id) {
                if support_choice > vote.choice_index_ho - vote.choice_index_lo {
                    return false;
                }
                // has voted
                if let Some(choice_id) = self.voters.get(&(vote_id, voter)) {
                    if *choice_id != support_choice {
                        let choice_vec_index = vote.choice_index_lo + *choice_id;
                        let choices = &mut self.choices;
                        choices.get_mut(choice_vec_index).unwrap().yea -= 1;
                        vote.support_num -= 1;
                    }
                } 
                let choices = &mut self.choices;
                let choice_vec_index = vote.choice_index_lo + support_choice;
                let voter_choice = choices.get_mut(choice_vec_index).unwrap();
                voter_choice.yea += 1;
                // record voter choice id
                self.voters.insert((vote_id, voter), support_choice);    
                vote.support_num += 1;
                self.env().emit_event(CastVote{
                    vote_id,
                    voter: self.env().caller(), 
                    support_choice,
                });
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn query_one_vote(&self, vote_id: VoteId) -> DisplayVote {
            assert!(self.vote_exists(vote_id));
            let vote = self.votes.get(&vote_id).unwrap(); 
            let display_vote = self.convert_vote_to_displayvote(&vote); 
            display_vote
        }

        #[ink(message)]
        pub fn query_all_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, vote) in &self.votes {
                let vote = self.convert_vote_to_displayvote(&vote);
                v.push(vote);
            }
            return v;
        }

        #[ink(message)]
        pub fn query_executed_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, val) in &self.votes {
                if self.is_vote_executed(&val) {
                    let vote = self.convert_vote_to_displayvote(&val);
                    v.push(vote);
                }
            }
            return v;
        }

        #[ink(message)]
        pub fn query_open_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, val) in &self.votes {
                if self.is_vote_open(&val) {
                    let vote = self.convert_vote_to_displayvote(&val);
                    v.push(vote);
                }
            }
            return v;
        }

        #[ink(message)]
        pub fn query_wait_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, val) in &self.votes {
                if self.is_vote_wait(&val) {
                    let vote = self.convert_vote_to_displayvote(&val);
                    v.push(vote);
                }
            }
            return v;
        }
 
        fn convert_vote_to_displayvote(&self, vote: &Vote) -> DisplayVote {
            let mut choices = Vec::new();
            let mut index = 0;
            let source_choices = &self.choices;
            for choice in source_choices.iter() {
                if index >= vote.choice_index_lo && index < vote.choice_index_ho {
                    let s = format!("{0}:{1}", choice.content.clone(), choice.yea);
                    choices.push(s);
                }
                index += 1;
            }
            let choices_content = choices.join("|"); 
            let vote = DisplayVote{
                vote_id: vote.vote_id,
                executed: vote.executed,
                title: vote.title.clone(),
                desc: vote.desc.clone(),
                start_date: vote.start_date,
                vote_time: vote.vote_time,
                support_require_num: vote.support_require_num,
                min_require_num: vote.min_require_num,
                support_num: vote.support_num,
                choices: choices_content,
            };
            vote
        }

        fn vote_exists(&self, vote_id: u64) -> bool {
            return vote_id < self.votes_length;
        }

        fn is_vote_open(&self, vote: &Vote) -> bool {
            return self.env().block_timestamp() < vote.start_date + vote.vote_time && !vote.executed;
        }

        fn is_vote_wait(&self, vote: &Vote) -> bool {
            return self.env().block_timestamp() > vote.start_date + vote.vote_time && !vote.executed;
        }

        fn is_vote_executed(&self, vote: &Vote) -> bool {
            return vote.executed;
        }

        fn is_vote_finished(&self, vote: &Vote) -> bool {
            return self.env().block_timestamp() < vote.start_date + vote.vote_time;
        }

        fn can_execute(&self, vote: &Vote) -> bool {

            if vote.executed {
                return false;
            }
            if self.is_vote_open(&vote) {
                return false;
            }
            if vote.support_num < vote.min_require_num {
                return false;
            }
            if vote.support_num == 0 {
                return false;
            }
            let mut index = 0;
            let choices = &self.choices;
            for choice in choices.iter() {
                if index >= vote.choice_index_lo && index < vote.choice_index_ho {
                    if choice.yea >= vote.support_require_num {
                        return true;
                    }
                }
                index += 1;
            }
            return false;
        }
    }

    #[cfg(test)]
    mod tests {
        use ink_lang as ink;

        use super::*;
        use ink_env::{
            call,
            test,
        };

        #[ink::test]
        fn test_split() {

            let choices = "A,B,C".to_string();
            let split = choices.split(",");
            ink_env::debug_println("hello");
            for s in split {
                ink_env::debug_println(&s);
            }
        }

        #[ink::test]
        fn test_split_with_vec() {
            let choices = "A,B,C".to_string();
            let vec: Vec<&str> = choices.split(",").collect();
            let i:u32 = 1;
            let length = i + vec.len() as u32;
            assert!(length == 4);
            for s in vec{
                ink_env::debug_println(&s);
            }
        }

        #[ink::test]
        fn test_calculate() {
            let choice: u64 = 3;
            let support: u64 = 5;
            let t : u64 = choice * 1000 / support; 
            ink_env::debug_println(t.to_string().as_str());
        }

        #[ink::test]
        fn new_vote_manager() {
            // let accounts =
            //     ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            //         .expect("Cannot get accounts");
            let vote_manager = VoteManager::new();

            assert_eq!(vote_manager.votes_length, 0);
        }

        // #[ink::test]
        // fn new_vote() {
        //     let accounts =
        //         ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        //             .expect("Cannot get accounts");
        //     let vote_manager = VoteManager::new();
            
        //     let r = vote_manager.new_vote("hello".to_string(), "hello world".to_string(), 100, 1, 0, "A,B,C,D".to_string());
        //     assert_eq!(r, 0);

        // }
    }
}