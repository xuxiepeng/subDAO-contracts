# SubDAO DAO-manager Module

DAO-manager is a module to manager DAO using components, like ERC20 ORG...

## Modules

### DAOTemplate
```rust
pub struct DAOTemplate {
    // template's id
    pub id: u64,
    // template's owner
    pub owner: AccountId,
    // template's name
    pub name: String,
    // template's dao manager
    pub dao_manager_code_hash: Hash,
    // components code hash
    // like { "ERC20": 0xqw...122, "ORG": 0xqw...123 }
    pub components: BTreeMap<String, Hash>,
}
```

### DAOComponentAddrs
```rust
pub struct DAOComponentAddrs {
    // base module contract's address
    base_addr: Option<AccountId>,
    // erc20 module contract's address
    erc20_addr: Option<AccountId>,
    // org module contract's address
    org_addr: Option<AccountId>,
    // vault module contract's address
    vault_addr: Option<AccountId>,
    // vote module contract's address
    vote_addr: Option<AccountId>,
    // github module contract's address
    github_addr: Option<AccountId>,
}
```

## Interfaces

### instance module
instance module.
```bash
type: tx
definition: pub fn new(controller: AccountId, org_id: u64, template: DAOTemplate) -> Self;
```

### init module
init module with erc20 params and vote params.
```bash
type: tx
definition: pub fn init(&mut self, base_name: String, base_logo: String, base_desc: String,
                    erc20_name: String, erc20_symbol: String, erc20_initial_supply: u64, erc20_decimals: u8) -> bool;
```

### query component addresses
query component addresses.
```bash
type: query
definition: pub fn query_component_addrs(&self) -> DAOComponentAddrs;
```

### transfer erc20 from owner
transfer erc20 to other from owner.
```bash
type: tx
definition: pub fn transfer(&mut self, to: AccountId, value: u64) -> bool;
```

### mint erc20 to account
mint erc20 by owner to account.
```bash
type: tx
definition: pub fn mint_token_by_owner(&mut self, to: AccountId, value: u64, ) -> bool;
```

### destroy erc20 to account
destroy erc20 by owner to account.
```bash
type: tx
definition: pub fn destroy_token_by_owner(&mut self, from: AccountId, value: u64) -> bool;
```

### add moderator to DAO
add moderator by owner to DAO.
```bash
type: tx
definition: pub fn add_dao_moderator(&mut self, name: String, moderator: AccountId) -> bool;
```

### remove moderator to DAO
remove moderator by owner to DAO.
```bash
type: tx
definition: pub fn remove_dao_moderator(&mut self, member: AccountId) -> bool;
```


