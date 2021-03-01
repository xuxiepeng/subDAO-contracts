# Vote Module

## Interface

### new() -> Self

Init a new vote module.

### new_vote(title: String, desc: String, vote_time: u64, support_require_num: u64, min_require_num: u64, choices: String) -> u64

Create a new vote.

params:

* title: the vote's title
* desc: the vote's desc
* vote_time: how long the vote durate by seconds.
* support_require_num: minimum support require numbers.
* min_require_num: minimum voter require numbers.
* choices: all vote choice, split by `,` , eg: A,B,C,D 

return

* vote_id


### vote(vote_id: VoteId, support_choice: u32, voter: AccountId) -> bool

Do a vote.

params:

* vote_id: a vote id, u64
* support_choice: which choice_id to be choosed, from zero. so, if there is four choices like A, B, C, D. Here 0 refers A, 1 refers B etc.
* voter: the voter account id

return:

* result: true/false

### execute(vote_id: VoteId)

execute a vote. 

mark status to executed.

### query_one_vote(vote_id: VoteId) -> DisplayVote

query vote by vote_id

if vote_id didn't exist, the function will runtime overhead.

### query_all_vote() -> alloc::vec::Vec<DisplayVote>

query all votes

### query_executed_vote() -> alloc::vec::Vec<DisplayVote>

query all executed votes.

### query_open_vote() -> alloc::vec::Vec<DisplayVote>

query all unfinshed votes.

### query_wait_vote() -> alloc::vec::Vec<DisplayVote>

query all finished but unexecuted votes.