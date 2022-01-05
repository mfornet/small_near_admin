use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner: AccountId,
    pub counter: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self { owner, counter: 0 }
    }

    pub fn set_owner(&mut self, owner: AccountId) {
        self.assert_called_by_owner();
        self.owner = owner;
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn get_counter(&self) -> U64 {
        self.counter.into()
    }

    pub fn increase_counter(&mut self) {
        self.assert_called_by_owner();
        self.counter += 1;
    }
}

impl Contract {
    fn called_by_owner(&self) -> bool {
        let caller = env::predecessor_account_id();
        caller == self.owner || caller == env::current_account_id()
    }

    fn assert_called_by_owner(&self) {
        assert!(self.called_by_owner())
    }
}
