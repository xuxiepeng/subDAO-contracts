# SubDAO Org Module
## Overview
Each DAO will have creator, moderator and normal member.

### new(_creator: AccountId,_orgId:u64):self

create a new org

### get_dao_creator():AccountId

get creator

### get_orgid(): u64 

get org id

### get_dao_moderator_list():Vec<AccountId>

get moderator list

### get_dao_members_list():Vec<AccountId>

get member list

### add_dao_moderator(name:String,moderator: AccountId):bool

add moderator

### add_dao_member(name:String,member: AccountId):bool

add member

### 2.8 remove_dao_moderator(name:String,moderator: AccountId):bool

remove moderator

### remove_dao_member(name:String,member: AccountId):bool

remove member

### resign(member: AccountId) -> bool

resgin from org

### get_dao_member_detail_list() -> alloc::vec::Vec<(AccountId, String)>

get member list, tuple

### get_dao_moderator_detail_list() -> alloc::vec::Vec<(AccountId, String)>

get moderator list, tuple

### TODO: transfer ownership

## Test

```
carge +nightly test
```

## Deploy

call `new(_creator: AccountId,_orgId:u64):self` with creator's address and org id.
