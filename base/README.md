# SubDAO Base Module

## Interfaces

### init_base(name: String, logo: String, desc: String)

init base module with name, logo url, desc.

### set_name(name: String)
set name of dao

### get_name() -> String
return the name of dao

### set_logo(logo: String)
set the logo url of dao

### get_logo() -> String
return the logo url of dao

### set_desc(desc: String)
set the description of dao

### get_desc() -> String
return the description of dao

### set_owner(owner: AccountId)
set the owner of dao

### get_owner() -> AccountId
return the owner of dao

## How to test

```
cargo +nightly test -- --nocapture
```
