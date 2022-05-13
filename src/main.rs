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
use store::{Entry, NewEntry};

use diesel::connection::SimpleConnection;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::dsl::sql_query;

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

    fn attempt_claim(&mut self, conn: &PgConnection) {
        use self::schema::queue::dsl::*;

        // Attempt to claim a resource
        let transaction = "SELECT * FROM claim_resource();";

            /*format!(
            "\
        BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE; \
        UPDATE queue SET owner={} WHERE id = \
                (SELECT id FROM queue \
                 WHERE owner=0 \
                 ORDER BY id \
                 LIMIT 1); \
        COMMIT;",
            &self.id
        );*/
        //let mut updated = conn.batch_execute(&transaction);
        let updated: Result<Vec<Entry>, Error> = sql_query(transaction).load(conn);
        /*self.claim_attempts += 1;
        while updated != Ok(_) {
            if self.claim_attempts >= 10 {
                break;
            };
            match updated {
                Err(Error::DatabaseError(DatabaseErrorKind::SerializationFailure, _)) => {
                    updated = conn.batch_execute(&transaction);
                    self.claim_attempts += 1;
                }
                _ => {
                    break;
                }
            }
        }*/
        println!("Result: {:?}", updated);

        // Check to see which resource(s) it claimed
        self.claimed_resources = queue
            .filter(owner.eq(*&self.id as i32))
            .load::<Entry>(conn)
            .expect("Error loading entries");
        println!("Claimed resources: {:?}", &self.claimed_resources);
    }
}

/*

TODO:
    Create tokio runtime
    Spawn a bunch of instances

*/

fn main() {
    use self::schema::queue::dsl::*;

    let connection = store::establish_connection();

    //store::create_update(&connection).unwrap();
    // let mut inst_1 = Instance::new(1);
    // inst_1.attempt_claim(&connection);

    /*let new_entry = NewEntry {
        owner: 0,
        food: "corndog".to_string(),
    };

    let entry = create_entry(&connection, new_entry);
    println!("Created new entry with id {}", entry.id);*/

    // store::reset_entries(&connection);

    let results = queue
        .limit(15)
        .load::<Entry>(&connection)
        .expect("Error loading entries");

    println!("Displaying {} entries", results.len());
    for entry in results {
        println!("");
        println!("Id: {:?}", entry.id);
        println!("Owner: {:?}", entry.owner);
        println!("Food: {:?}", entry.food);
    }
}
/*
#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (mut client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let update = client
        .build_transaction()
        .isolation_level(IsolationLevel::Serializable)
        .start().await?
        .query("\
        UPDATE queue \
        SET owner = 8 \
        WHERE id = 10
        RETURNING *; \
        COMMIT;", &[]).await?;

    for entry in update {
        let id: i32 = entry.try_get("id")?;
        let owner: i32 = entry.try_get("owner")?;
        let food: &str = entry.try_get("food")?;
        println!("");
        println!("Id: {:?}", id);
        println!("Owner: {:?}", owner);
        println!("Food: {:?}", food);
    }

    let rows = client.query("SELECT * FROM queue;", &[]).await?;
    
    for entry in rows {
        let id: i32 = entry.try_get("id")?;
        let owner: i32 = entry.try_get("owner")?;
        let food: &str = entry.try_get("food")?;
        println!("");
        println!("Id: {:?}", id);
        println!("Owner: {:?}", owner);
        println!("Food: {:?}", food);
    }


    //let value: &str = rows[0].get(0);

    Ok(())
}*/
