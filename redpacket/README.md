# SubDAO RedPacket Module

## Structures
### red packet struct
```rust
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
```

## Interfaces

### create_red_packet(token_type: u8, if_random: u8, total_number: u64,
                                 end_time: u64, token_addr: AccountId, total_tokens: u64, password: Hash) -> bool

create red packet
params:
token_type, 0 for unit, 1 for erc20, 0 is not support for now;
if_random, 0 distribute avg, 1 random;
total_number, claim number;
end_time, claim end time;
token_addr, erc20 address;
total_tokens, deposit tokens number;
password, for claim to verify;

### claim(id: u64, password: Hash, recipient: AccountId) -> bool

claim red packet
params:
id, id of red packet;
password, for claim to verify;
recipient, claim to address;

### check_red_packet(id: u64) -> RedPacket

query red packet
params:
id, id of red packet;

### refund(id: u64) -> bool

refund red packet, only by red packet creator;
params:
id, id of red packet;

## How to test

```
cargo +nightly test -- --nocapture
```
