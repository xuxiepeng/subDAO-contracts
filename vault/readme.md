# SubDAO Valut Module
## 1.模块概述
DAO 组织的运营是需要资金支持的，国库模块主要用于 DAO 资金的管理。

## 2.接口描述

### 2.1 new()
+ 对国库进行初始化。涉及到一些数据结构的初始化。

### 2.2 add_vault_token()
+ 增加一种token

### 2.3 remove_vault_token()
+ 移除一种token。移除只是从 token可见列表(visible_tokens)中移除，在tokens中该币仍然存在。

### 2.4 get_token_list()
+ 返回当前的可见的token 列表。只有可见的token ,才能给组织内的成员奖励。

### 2.5 get_balance_of()
+ 返回某一个token的余额。

### 2.6 deposit()
+ 把某一种token的 指定数量的资金存入国库

### 2.7 withdraw()
+ 把某一种token的 指定数量的资金转出国库,并奖励给某位成员

### 2.8 get_transfer_history()
+ 返回转账的流水

## 3 测试用例
+ 写了一些测试用例，来测试接口的功能。

## 4  todo:
+ 从 外部账号给国库转账，或从 国库给外部转账时，暂时没想好如何对 外部账号的余额进行操作。
见 226，268 行的注释

