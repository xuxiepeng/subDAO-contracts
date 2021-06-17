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
definition: pub fn init_by_params(&mut self, params: DAOInitParams) -> bool;
if you want init erc20, you must init vault;
if you want init vote, you must init vault;
if you want init vault, you must init org and auth;
param description:
{
    "base": {
        "owner": "5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF",
        "name": "test",
        "logo": "xx",
        "desc": "xx"
    },
    "erc20": {
        "owner": "5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF",
        "name": "xx",
        "symbol": "xx",
        "totalSupply": 100,
        "decimals": 0
    },
    "erc20Transfers": [
        ["5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF", 100],
        ["5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF", 100],
    ],
    "org": {
        "owner": "5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF",
        "moderators": [
            ["xxx", "5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF"],
            ["xxx", "5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF"]
        ]
    },
    "auth": {
        "owner": "5CZLWjNc7LGb6QfRie8YLMwULdcJif2HBUJ52TwaxpMyxPrF",
    }
}
```

### query component addresses
query component addresses.
```bash
type: query
definition: pub fn query_component_addrs(&self) -> DAOComponentAddrs;
```


