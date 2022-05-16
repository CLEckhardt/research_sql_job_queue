//
// WHAT ARE WE TESTING
//
//
//
// WHAT DO WE EXPECT TO SEE
//
//
//
// HOW DO WE KNOW IF THIS WON'T WORK
//
//
//

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
mod store;

use diesel::prelude::*;
use dotenv::dotenv;
use schema::queue;
use std::env;
use store::{Entry, NewEntry, PgStore};

use diesel::connection::SimpleConnection;
use diesel::dsl::sql_query;
use diesel::result::{DatabaseErrorKind, Error};

//use tokio_postgres::{IsolationLevel, Transaction, NoTls, Error};

struct Instance {
    id: u16,
    claim: Option<u16>,
    claimed_resources: Vec<Entry>,
    claim_attempts: u16,
}

impl Instance {
    fn new(id: u16) -> Self {
        Self {
            id,
            claim: None,
            claimed_resources: Vec::new(),
            claim_attempts: 0,
        }
    }

    fn attempt_claim(&mut self, store: &PgStore) {
        use self::schema::queue::dsl::*;

        // Attempt to claim a resource

        let mut updated = store.execute_attempt(&self.id);
        self.claim_attempts += 1;

        while updated.is_err() {
            if self.claim_attempts >= 10 { break };
            match updated {
                // Retry on serialization error
                Err(Error::SerializationError(_)) => {
                    updated = store.execute_attempt(&self.id);
                    self.claim_attempts += 1;
                }
                _ => break,
            };
        };

        println!("Result: {:?}", updated);
    }
}

/*

TODO:
    Create tokio runtime
    Spawn a bunch of instances

*/

fn main() {
    use self::schema::queue::dsl::*;

    let store = PgStore::new();

    let mut inst_1 = Instance::new(2);
    inst_1.attempt_claim(&store);


    //store::reset_entries();
    store.print_all();

}
