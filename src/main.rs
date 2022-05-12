#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
mod store;

use diesel::prelude::*;
use dotenv::dotenv;
use schema::queue;
use std::env;
use store::{Entry, NewEntry};

use diesel::connection::SimpleConnection;

struct Instance {
    id: u16,
    claim: Option<u16>,
    claimed_resource: Option<String>,
    claim_attempts: u16,
}

impl Instance {
    fn new(id: u16) -> Self {
        Self {
            id,
            claim: None,
            claimed_resource: None,
            claim_attempts: 0,
        }
    }

    fn attempt_claim(&mut self, conn: &PgConnection) {
        use self::schema::queue::dsl::*;
        let updated = conn.batch_execute(&format!(
            "\
        BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE; \
        UPDATE queue SET owner={} WHERE id = \
                (SELECT id FROM queue \
                 WHERE owner=0 \
                 ORDER BY id \
                 LIMIT 1); \
        COMMIT;",
            &self.id
        ));
        println!("Result: {:?}", updated);
    }
}

/*
pub fn reset_entries(conn: &PgConnection) {
    use self::schema::queue::dsl::*;
    let reset = diesel::update(queue).set(owner.eq(0)).execute(conn);
*/
fn main() {
    use self::schema::queue::dsl::*;

    let connection = store::establish_connection();

    //let mut inst_5 = Instance::new(5);
    //inst_5.attempt_claim(&connection);

    /*let new_entry = NewEntry {
        owner: 0,
        food: "corndog".to_string(),
    };

    let entry = create_entry(&connection, new_entry);
    println!("Created new entry with id {}", entry.id);*/

    //store::reset_entries(&connection);

    let results = queue
        .limit(15)
        .load::<Entry>(&connection)
        .expect("Error loading entries");

    println!("Displaying {} entries", results.len());
    for entry in results {
        println!("Id: {:?}", entry.id);
        println!("Owner: {:?}", entry.owner);
        println!("Food: {:?}", entry.food);
    }
}
