#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use schema::queue;
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_entry(conn: &PgConnection, new_entry: NewEntry) -> Entry {
    diesel::insert_into(queue::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving new entry")
}

fn reset_entries(conn: &PgConnection) {
    use self::schema::queue::dsl::*;
    let reset = diesel::update(queue).set(owner.eq(0)).execute(conn);
    println!("Reset {:?} entries", reset);
}

#[derive(Queryable)]
struct Entry {
    id: i32,
    owner: i32,
    food: String,
}

#[derive(Insertable)]
#[table_name = "queue"]
struct NewEntry {
    owner: i32,
    food: String,
}

fn main() {
    use self::schema::queue::dsl::*;

    let connection = establish_connection();

    /*let new_entry = NewEntry {
        owner: 0,
        food: "corndog".to_string(),
    };

    let entry = create_entry(&connection, new_entry);
    println!("Created new entry with id {}", entry.id);*/

    reset_entries(&connection);

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
