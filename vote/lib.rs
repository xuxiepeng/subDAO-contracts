#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod vote_manager {

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    type VoteId = u64;

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
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Vote {
        executed: bool,
        start_date: u64,
        vote_time: u64,
        support_require_pct: u64,
        yea: u64,
        nay: u64,
    }

    #[ink(storage)]
    pub struct VoteManager {
        support_require_pct: u64,
        min_require_num: u64,
        votes_length: u64,
        vote_time: u64,
        votes: StorageHashMap<VoteId, Vote>,
        voters: StorageHashMap<(VoteId, AccountId), VoterState>,
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

        support: bool,
    }

    #[ink(event)]
    pub struct ExecuteVote {
        #[ink(topic)]
        vote_id: VoteId,
    }

    impl VoteManager {

        #[ink(constructor)]
        pub fn new(_vote_time: u64, support_require_pct: u64, min_require_num: u64) -> Self {
            Self { 
                support_require_pct,
                min_require_num,
                votes_length: 0,
                vote_time: _vote_time,
                votes: StorageHashMap::default(),
                voters: StorageHashMap::default(),
            }
        }

        #[ink(message)]
        pub fn new_vote(&mut self) {
            let vote_id = self.votes_length.clone();
            self.votes_length += 1;
            let start_date: u64 = self.env().block_timestamp();
            let vote = Vote{
                executed: false,
                start_date: start_date,
                vote_time: self.vote_time.clone(),
                support_require_pct: self.support_require_pct.clone(),
                yea: 0,
                nay: 0,
            };
            self.votes.insert(vote_id, vote);
            self.env().emit_event(StartVote{
                vote_id,
                creator: self.env().caller(),
            });
        }

        #[ink(message)]
        pub fn vote(&mut self, vote_id: VoteId,  support: bool, voter: AccountId) {
            assert!(self.vote_exists(vote_id));
            if let Some(_vote) = self.votes.get_mut(&vote_id) {
                if let Some(vote_state) = self.voters.get(&(vote_id, voter)) {
                    match vote_state {
                        VoterState::Yea => {
                            _vote.yea -= 1;
                        },
                        VoterState::Nay => {
                            _vote.nay -= 1;
                        },
                        VoterState::Absent => (),
                    }
                }
                if support {
                    _vote.yea += 1;
                    self.voters.insert((vote_id, voter), VoterState::Yea);
                } else {
                    _vote.nay += 1;
                    self.voters.insert((vote_id, voter), VoterState::Nay);
                }
                self.env().emit_event(CastVote{
                    vote_id,
                    voter: self.env().caller(), 
                    support,
                });
            }
        }

        #[ink(message)]
        pub fn next_index(&self) -> u64 {
            self.votes_length
        }

        fn vote_exists(&self, vote_id: u64) -> bool {
            return vote_id < self.votes_length;
        }

        fn is_vote_open(&self, vote: Vote) -> bool {
            return self.env().block_timestamp() < vote.start_date + self.vote_time && !vote.executed;
        }
    }
}
