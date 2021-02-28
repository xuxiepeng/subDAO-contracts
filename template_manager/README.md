# SubDAO template-manager Module

template-manager is a module to manager templates.

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
