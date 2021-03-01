# SubDAO main Module

main is a module to manager templates and instance DAO.

## Modules

### DAOInstance
```rust
pub struct DAOInstance {
        id: u64,
        owner: AccountId,
        dao_manager: DAOManager,
        dao_manager_addr: AccountId,
    }
```

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

## Interfaces

### instance module
instance module.
```bash
type: tx
definition: pub fn new(controller: AccountId) -> Self;
```

### init module
init module with template code hash.
```bash
type: tx
definition: pub fn init(&mut self, template_code_hash: Hash) -> bool;
```

### add template
add template with DAO-manager code hash and component's code hash, like ERC20, ORG...
```bash
type: tx
definition: pub fn add_template(&mut self, name: String, dao_manager_code_hash: Hash, components: BTreeMap<String, Hash>) -> bool;
```

### list templates
list templates.
```bash
type: query
definition: pub fn list_templates(&self) -> Vec<DAOTemplate>;
```

### query templates by index
query templates by index.
```bash
type: query
definition: pub fn query_template_by_index(&self, index: u64) -> DAOTemplate;
```

### query template addr
query template addr.
```bash
type: query
definition: pub fn query_template_addr(&self) -> AccountId;
```

### query template addr by owner
query template addr by owner.
```bash
type: query
definition: pub fn list_dao_instances_by_owner(&mut self, owner: AccountId) -> Vec<DAOInstance>;
```

### instance DAO by template
instance DAO by template, input template's index and DAO controller.
```bash
type: tx
definition: pub fn instance_by_template(&mut self, index: u64, controller: AccountId) -> bool;
```
