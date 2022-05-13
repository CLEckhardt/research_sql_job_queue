use crate::schema::*;

use super::*;
use diesel::prelude::*;
use dotenv::dotenv;
use schema::queue;
use std::env;
use diesel::result::Error;


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

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_entry(conn: &PgConnection, new_entry: NewEntry) -> Entry {
    diesel::insert_into(queue::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving new entry")
}

pub fn reset_entries(conn: &PgConnection) {
    use self::schema::queue::dsl::*;
    let reset = diesel::update(queue).set(owner.eq(0)).execute(conn);
    println!("Reset {:?} entries", reset);
}

pub fn create_update(conn: &PgConnection) -> Result<(), Error> {
    conn.batch_execute("\
    CREATE OR REPLACE FUNCTION claim_resource(claimer integer) \
    RETURNS TABLE(id integer, owner integer, resource varchar(48)) AS $$ \
    BEGIN SET TRANSACTION ISOLATION LEVEL SERIALIZABLE; \
        UPDATE queue SET owner=claimer WHERE id = \
                (SELECT id FROM queue \
                 WHERE owner=0 \
                 ORDER BY id \
                 LIMIT 1)
                 RETURNING *; \
        END; $$ LANGUAGE plpgsql;
        ")
}
