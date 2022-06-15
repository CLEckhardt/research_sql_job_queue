use crate::schema::queue::dsl::*;
use crate::schema::*;

use super::*;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use schema::queue;
use std::env;

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "queue"]
pub struct Entry {
    pub id: i32,
    pub owner: i32,
    pub food: String,
}

#[derive(Insertable)]
#[table_name = "queue"]
pub struct NewEntry {
    pub owner: i32,
    pub food: String,
}

pub struct PgStore {
    connection: PgConnection,
}

impl PgStore {
    pub fn new() -> Self {
        Self {
            connection: Self::establish_connection(),
        }
    }

    fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    pub fn create_entry(&self, new_entry: NewEntry) -> Entry {
        diesel::insert_into(queue::table)
            .values(&new_entry)
            .get_result(&self.connection)
            .expect("Error saving new entry")
    }

    pub fn reset_entries(&self) {
        use self::schema::queue::dsl::*;
        let reset = diesel::update(queue)
            .set(owner.eq(0))
            .execute(&self.connection);
        println!("Reset {:?} entries", reset);
    }

    pub fn print_all(&self) {
        let results = queue
            .limit(15)
            .load::<Entry>(&self.connection)
            .expect("Error loading entries");

        println!("Displaying {} entries", results.len());
        for entry in results {
            println!("");
            println!("Id: {:?}", entry.id);
            println!("Owner: {:?}", entry.owner);
            println!("Food: {:?}", entry.food);
        }
    }

    pub fn execute_attempt(&self, instance_id: &u16) -> Result<Vec<Entry>, Error> {
        let transaction = format!(
            "UPDATE queue SET owner={} WHERE id = \
                (SELECT id FROM queue \
                 WHERE owner=0 \
                 ORDER BY id \
                 LIMIT 1)
                RETURNING *;",
            instance_id
        );

        self.connection
            .build_transaction()
            .serializable()
            .run(|| sql_query(&transaction).load(&self.connection))


    }

    pub fn execute_attempt_control(&self, instance_id: &u16) -> Result<Vec<Entry>, Error> {
        let transaction = format!(
            "UPDATE queue SET owner={} WHERE id = \
                (SELECT id FROM queue \
                 WHERE owner=0 \
                 ORDER BY id \
                 LIMIT 1)
                RETURNING *;",
            instance_id
        );

        self.connection
            .build_transaction()
            .run(|| sql_query(&transaction).load(&self.connection))


    }
}
