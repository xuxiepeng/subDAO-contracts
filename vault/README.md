# SubDAO Valut Module
## 1.模块概述
DAO 组织的运营是需要资金支持的，国库模块主要用于 DAO 资金的管理。

## 2.接口描述

### 2.1 init(vault_contract_address: AccountId)
+ 对国库进行初始化。保存当前国库合约的地址， 便于 后续erc20 转账使用

### 2.2 add_vault_token(erc_20_address:AccountId) -> bool
+ 增加一种token。erc_20_address: 该 erc 20 的合约地址。

### 2.3 remove_vault_token(erc_20_address: AccountId) -> bool
+ 移除一种token。移除只是从 token可见列表(visible_tokens)中移除，在tokens中该币仍然存在。

### 2.4 get_token_list()：vec::Vec<AccountId>
+ 返回当前的可见的token 列表。只有可见的token ,才能给组织内的成员奖励。

### 2.5 get_balance_of(erc_20_address: AccountId) -> u64
+ 返回某一个token的余额。erc_20_address: 该 erc 20 的合约地址。 加了一个权限控制（只允许查询 “注册tokens 列表” 中的 erc20 token 的余额）

### 2.6 deposit(erc_20_address:AccountId, from_address:AccountId,value:u64) -> bool
+ 把某一种token的 指定数量的资金存入国库,目前只允许 往 “注册tokens 列表” 里 的 币转账。
  from_address:从该账号地址转入国库。
  erc_20_address: 该 erc 20 的合约地址。
    

### 2.7 withdraw(erc_20_address:AccountId,to_address:AccountId,value:u64) -> bool
+ 把某一种token的 指定数量的资金转出国库,并奖励给某位成员。把资金转出国库，目前只允许 从 “可见 tokens 列表” 里的币 转出。同时， 只有管理员或者creator ,可以转出资金。
  to_address:从国库的资金转入该账号地址。
  erc_20_address: 该 erc 20 的合约地址。

### 2.8 get_transfer_history()：vec::Vec<Transfer> 
+ 返回转账的流水。

## 3 测试用例
+ 写了一些测试用例，来测试接口的功能。

## 4  todo:


