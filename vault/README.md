# SubDAO Valut Module
## Overview

Valut for a DAO

### new(org_contract_address: AccountId)

Create

### add_vault_token(erc_20_address:AccountId) -> bool

add a token contract to support list

### remove_vault_token(erc_20_address: AccountId) -> bool

remove a token contract from support list

### get_token_list()：vec::Vec<AccountId>

get supported token list

### get_balance_of(erc_20_address: AccountId) -> u64

get token balance of currect vault

### deposit(erc_20_address:AccountId, from_address:AccountId,value:u64) -> bool

deposit token
    
### withdraw(erc_20_address:AccountId,to_address:AccountId,value:u64) -> bool

withdraw token from the specific token contract to account. The token contract should be in the support list.

### get_transfer_history()：vec::Vec<Transfer> 

get transfer history

## Test

```
cargo +nightly test
```

## Deploy

call `new(org_contract_address: AccountId)` with the org address.


