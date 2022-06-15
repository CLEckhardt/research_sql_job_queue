#[allow(dead_code, unused_must_use, unused_imports, unused_variables)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod control;
mod experiment;
pub mod schema;
mod store;

use std::sync::{Arc, Mutex};

use log::debug;

use diesel::prelude::*;
use dotenv::dotenv;
use schema::queue;
use std::env;
use store::{Entry, NewEntry, PgStore};

use diesel::connection::SimpleConnection;
use diesel::dsl::sql_query;
use diesel::result::{DatabaseErrorKind, Error};

//use tokio_postgres::{IsolationLevel, Transaction, NoTls, Error};

/*

TODO:
    Create tokio runtime
    Spawn a bunch of instances

*/

fn main() {
    use self::schema::queue::dsl::*;

    env_logger::init();

    let store = PgStore::new();

    //let registry: Arc<Mutex<Vec<experiment::Instance>>> =
    //    Arc::new(Mutex::new(Vec::new()));
    let registry: Arc<Mutex<Vec<experiment::Instance>>> = Arc::new(Mutex::new(Vec::new()));
    let temp_registry = Arc::clone(&registry);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("tokio_runtime")
        .enable_all()
        .build()
        .unwrap();

    let (tx, rx) = tokio::sync::watch::channel("wait");

    let handle = std::thread::Builder::new()
        .name("runtime_thread".to_string())
        .spawn(move || {
            runtime.block_on(async move {
                for i in 1..=20 {
                    tokio::spawn(experiment::Instance::spawn(
                        i,
                        PgStore::new(),
                        rx.clone(),
                        Arc::clone(&temp_registry),
                    ));
                    debug!("Spawned task: {}", i);
                }
                tokio::time::sleep(std::time::Duration::from_secs(4)).await;
            })
        })
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    debug!("GO!!!");
    let sent = tx.send("go");

    handle.join();

    let _ = sent.unwrap();

    store.print_all();
    {
        let results = registry.lock().unwrap();
        for r in &*results {
            if r.claim_attempts < 11 {
                println!("Result: {:?}", r);
            }
        }
    }
    store.reset_entries();
    //println!("Result: {:?}", experiment_registry);
}
