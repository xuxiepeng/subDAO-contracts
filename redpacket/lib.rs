#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::red_packet_manager::RedPacketManager;

#[ink::contract]
mod red_packet_manager {

    use alloc::string::String;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };
    use ink_prelude::collections::BTreeMap;
    use erc20::Erc20;

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct RedPacket {
        id: u64,
        creator: AccountId,
        // red packet allow claim number
        total_number: u64,
        // red packet user who claimed number
        claimed_number: u64,
        // red packet token type, 0 unit, 1 erc20
        token_type: u8,
        // red packet if use random, 0 no use random, 1 use random
        if_random: u8,
        // red packet token address
        token_addr: AccountId,
        // red packet password
        password: Hash,
        // red packet remaining tokens
        remaining_tokens: u64,
        // red packet start time
        start_time: u64,
        end_time: u64,
        // claim tokens map by address
        claim_list: BTreeMap<AccountId, u64>,
        // is refund by creator
        is_refund: bool,
    }
    
    #[ink(storage)]
    pub struct RedPacketManager {
        // contract creator
        owner: AccountId,
        // red packet index
        index: u64,
        // random seed, gen when contract created
        seed: u64,
        // red packet map
        red_packet_map: StorageHashMap<u64, RedPacket>,
    }

    #[ink(event)]
    pub struct CreatedRedPacket {
        #[ink(topic)]
        id: u64,
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        token_type: u8,
        #[ink(topic)]
        total_tokens: u64,
    }

    impl RedPacketManager {

        #[ink(constructor)]
        pub fn new(owner: AccountId, magic: u64) -> Self {
            Self {
                owner,
                index: 0,
                // cannot put code
                // seed:Self::env().block_timestamp() * magic,
                seed: magic,
                red_packet_map: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn create_red_packet(&mut self, token_type: u8, if_random: u8, total_number: u64,
                                 end_time: u64, token_addr: AccountId, total_tokens: u64, password: Hash) -> bool {
            let from = self.env().caller();
            assert!(self.index + 1 > self.index, "cannot create red packet more");
            assert!(token_type == 0 || token_type == 1, "wrong token type");
            assert!(if_random == 0 || if_random == 1, "wrong token type");

            let start_time = self.env().block_timestamp();
            let contract_addr = self.env().account_id();
            self.index += 1;
            assert!(start_time < end_time, "end time wrong");

            let mut token_addr = token_addr;
            let claimed_number = 0;
            // check balance
            if token_type == 0 {
                // TODO token_type = 0 always trapped
                // token_type 0 only allowed by owner
                assert!(from != self.owner, "token_type 0 only allowed by owner");
                // assert!(u128::from(total_tokens) <= self.env().balance(), "no enough unit");
                // token_addr = AccountId::from([0x00; 32]);
            } else {
                let mut token :Erc20 = ink_env::call::FromAccountId::from_account_id(token_addr);
                assert!(token.transfer_from(from, contract_addr, total_tokens), "transfer_from token failed");
            }
            self.red_packet_map.insert(self.index, RedPacket {
                id: self.index,
                creator: from,
                total_number,
                claimed_number,
                token_type,
                if_random,
                token_addr,
                password,
                remaining_tokens: total_tokens,
                start_time,
                end_time,
                claim_list: BTreeMap::new(),
                is_refund: false,
            });
            // TODO emit event get trapped
            // self.env().emit_event(CreatedRedPacket {
            //     id: self.index,
            //     creator: from,
            //     token_type,
            //     total_tokens,
            // });
            true
        }

        #[ink(message)]
        pub fn claim(&mut self, id: u64, password: Hash, recipient: AccountId) -> bool {
            let from = self.env().caller();
            let cur_time = self.env().block_timestamp();
            let mut red_packet = self.red_packet_map.get_mut(&id).unwrap();
            let total_number = red_packet.total_number;
            let claimed_number = red_packet.claimed_number;
            let remaining_tokens = red_packet.remaining_tokens;
            let token_addr = red_packet.token_addr;

            assert!(!red_packet.is_refund, "refund by creator");
            assert!(cur_time < red_packet.end_time, "out of date");
            assert!(claimed_number < total_number, "out of stock");
            let claim_opt = red_packet.claim_list.get(&from);
            assert!(claim_opt.is_none(), "user have claimed");
            assert!(red_packet.password == password, "wrong password");

            // calculate claim cost
            let mut claimed_tokens = 0;
            if total_number - claimed_number <= 1 {
                claimed_tokens = remaining_tokens;
            } else if red_packet.if_random == 0 {
                // if no random, average tokens
                claimed_tokens = remaining_tokens / (total_number - claimed_number);
            } else if red_packet.if_random == 1 {
                // if random, random 1 - (remaining_tokens * 2 / (total_number - claimed_number) - 1)
                // TODO cannot use mul?
                claimed_tokens = (self.seed + red_packet.id + cur_time + 1) % ((remaining_tokens + remaining_tokens) / (total_number - claimed_number))
            }

            // modify red packet
            red_packet.remaining_tokens = remaining_tokens - claimed_tokens;
            red_packet.claim_list.insert(from, claimed_tokens);
            red_packet.claimed_number += 1;

            // transfer tokens
            if red_packet.token_type == 0 {
                // TODO claim unit
            } else if red_packet.token_type == 1 {
                let mut token :Erc20 = ink_env::call::FromAccountId::from_account_id(token_addr);
                assert!(token.transfer(recipient, claimed_tokens), "transfer token failed");
            }

            // TODO event
            true
        }

        #[ink(message)]
        pub fn refund(&mut self, id: u64) -> bool {
            let from = self.env().caller();
            let cur_time = self.env().block_timestamp();
            let mut red_packet = self.red_packet_map.get_mut(&id).unwrap();
            let remaining_tokens = red_packet.remaining_tokens;
            let token_addr = red_packet.token_addr;

            assert!(!red_packet.is_refund, "refund by creator");
            // disable temp
            // assert!(cur_time >= red_packet.end_time, "not out of date");
            assert!(remaining_tokens != 0, "out of tokens");
            assert!(from == red_packet.creator, "you are not creator");

            // transfer tokens
            if red_packet.token_type == 0 {
                // TODO claim unit
            } else if red_packet.token_type == 1 {
                let mut token :Erc20 = ink_env::call::FromAccountId::from_account_id(token_addr);
                assert!(token.transfer(from, remaining_tokens), "transfer token failed");
            }
            red_packet.is_refund = true;
            // TODO event
            true
        }

        #[ink(message)]
        pub fn check_red_packet(&self, id: u64) -> RedPacket {
            self.red_packet_map.get(&id).unwrap().clone()
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner.clone()
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
        fn test_create_red_packet() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut manager = RedPacketManager::new(accounts.alice, 11222222);

            assert!(manager.create_red_packet(0, 1, 100, 1111111111, AccountId::from([0x00; 32]), 0), "create failed");
        }

    }
}
