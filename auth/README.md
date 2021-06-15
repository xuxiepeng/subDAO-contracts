# SubDAO Auth Module

## Interface

### new(owner: AccountId) -> self

Init a new auth module.

### has_permission(account_id: AccountId, contract_name: String, function_name: String) -> bool

Whether an account has permission to access a function.

params:

* account_id: the account
* contract_name: the contract name
* function_name: the function name

return bool

### grant_permission(account_id: AccountId, contract_name: String, function_name: String) ->  bool 

Grant an account to access specific function

params:

* account_id: the account
* contract_name: the contract name
* function_name: the function name

### transfer_owner(to: AccountId)

Transfer owner to another

params

* to: the account

### revoke_permission(account_id: AccountId, contract_name: String, function_name: String) -> bool

Revoke an account's permission to access specific function

* account_id: the account
* contract_name: the contract name
* function_name: the function name

### register_action(contract_name: String, function_name: String, action_title: String) -> bool

Register an action on specific function

* contract_name: the contract name
* function_name: the function name
* action_title: the action name, like `read_dao`

### cancel_action(contract_name: String, function_name: String) -> bool

Cancel one action

* contract_name: the contract name
* function_name: the function name

### show_actions_by_contract(contract_name: String) ->  Vec<Action>

show actions by contract

* contract_name: the contract name

### show_actions_by_user(owner: AccountId) -> Vec<Action>

show actions of an account

* owner: the account