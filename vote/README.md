# Vote Module

## Interface

### new(_vote_time: u64, support_require_pct: u64, min_require_num: u64)

Init a new vote module.

_vote_time: the vote duration seconds. 
support_require_pct: minimum support require percentage. ex. 75% -> 7500.
min_require_num: minimum voter require numbers.

### new_vote()

Create a new vote.

### vote(vote_id: VoteId,  support: bool, voter: AccountId)

Do a vote.

vote_id: a vote id, u64
support: true or false refer to yes or no
voter: the voter account id

## Todo

* interaces of query