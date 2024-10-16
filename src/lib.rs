use near_sdk::{near, AccountId};

#[near(contract_state)]
pub struct Contract {}

impl Default for Contract {
    fn default() -> Self {
        Self {}
    }
}

#[near]
impl Contract {
    pub fn get_account_lockups(&self, account_id: AccountId) -> Vec<()> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_account_lockups() {
        let contract = Contract::default();
        let alice: AccountId = "alice.near".parse().unwrap();

        assert_eq!(contract.get_account_lockups(alice), vec![]);
    }
}
