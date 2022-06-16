#[macro_use]
extern crate diesel;
extern crate dotenv;

#[cfg(feature = "control")]
mod control;
#[cfg(feature = "experiment")]
mod experiment;
pub mod schema;
mod store;

use std::sync::{Arc, Mutex};

use log::debug;

use store::PgStore;

use diesel::dsl::sql_query;

fn main() {
    env_logger::init();

    let store = PgStore::new();

    #[cfg(feature = "control")]
    let registry: Arc<Mutex<Vec<control::Instance>>> = Arc::new(Mutex::new(Vec::new()));
    #[cfg(feature = "experiment")]
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
                for i in 1..=12 {
                    #[cfg(feature = "control")]
                    tokio::spawn(control::Instance::spawn(
                        i,
                        PgStore::new(),
                        rx.clone(),
                        Arc::clone(&temp_registry),
                    ));
                    #[cfg(feature = "experiment")]
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

    let _ = handle.join();

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
}
