use rand::Rng;

#[derive(Debug)]
pub enum EventType {
    AccountCreated {
        account_id: u32
    },
    WalletCreated {
        wallet_id: u32
    },
    WalletDeleted {
        wallet_id: u32
    },
    PerformedTransaction {
        from_id: u32,
        to_id: u32,
        amount: u32
    }
}

#[derive(Debug)]
pub struct Event {
    id: u32,
    event_type: EventType,
}

#[derive(Debug)]
pub struct Database {
    id: u32,
    table: Vec<Event>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            id: rand::thread_rng().gen_range(10000..99999),
            table: Vec::new()
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.table.push(event);
    }
}
