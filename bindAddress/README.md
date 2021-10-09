# SubDAO BindingAddress Module

## Interfaces

Bind ethereum address to caller's account
```
bind(eth:String) -> bool
```

Query my eth address
```
my_eth_address() -> String
```

Check if the address binded.
```
is_bind(address:AccountId) -> bool
```

Check if caller already bind ethereum address
```
is_me_bind() -> bool
```

List binded addresses
```
list_addresses(pageIndex:u64, pageSize:u64) -> Vec<PageResult>
```