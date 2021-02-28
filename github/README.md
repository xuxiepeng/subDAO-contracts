# Github Module



## Interface

### new_pull_request_auditor(& mut self, repo_url: String, pr_number: u64, github_id: u64, account_id: AccountId, auditor_id: AccountId)

Init a new github pull request module.

repo_url: The url of the github repository where the PR is located. 
pr_number: The number of the PR. 
github_id: The github user id of the PR owner.
account_id: The account id of the PR owner. 
auditor_id: The auditor id of the PR. 

### query_pull_request_audit_status(&self, index: Index ) 

Query pull request audit status.

### audit_pull_request(& mut self, index: Index, audit_result: bool )

Auditor review pull requst status.