use rand::Rng;

#[derive(Debug)]
pub struct Database<T> {
    id: u32,
    table: Vec<T>,
}

#[derive(Debug)]
pub enum EventType {
    AccountCreated {
        account_id: u32,
    },
    AccountDeleted {
        account_id: u32,
    },
    WalletCreated {
        wallet_id: u32,
    },
    WalletDeleted {
        wallet_id: u32,
    },
    PerformedTransaction {
        from_id: u32,
        to_id: u32,
        amount: u32,
    },
}

#[derive(Debug)]
pub struct Event {
    id: u32,
    event_type: EventType,
}

impl<T> Database<T> {
    pub fn new() -> Self {
        Database {
            id: rand::thread_rng().gen_range(10000..99999),
            table: Vec::new()
        }
    }

    pub fn add_event(&mut self, event: T) {
        self.table.push(event);
    }
}
