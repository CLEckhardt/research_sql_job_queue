
use log::debug;
use crate::store::{Entry, NewEntry, PgStore};
use diesel::result::{DatabaseErrorKind, Error};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Instance {
    pub idn: u16,
    //claim: Option<u16>,
    pub claimed_resources: Option<Vec<Entry>>,
    pub claim_attempts: u16,
}

impl Instance {
    pub async fn spawn(
        idn: u16,
        store: PgStore,
        mut rx: tokio::sync::watch::Receiver<&str>,
        registry: Arc<Mutex<Vec<Instance>>>,
    ) {
        use crate::schema::queue::dsl::*;
        // Wait for the go signal
        if rx.changed().await.is_ok() {
            let mut claim_attempts = 0;
            let mut updated: Result<Vec<Entry>, Error> = Ok(Vec::new());

            loop {
                // Attempt a claim
                updated = store.execute_attempt_control(&idn);
                debug!("Claim attempted by {}", &idn);
                claim_attempts += 1;
                match &updated {
                    Ok(v) => {
                        // Break if claim was successful
                        if v.len() > 0 {
                            break;
                        };
                    }
                    // Retry on serialization error
                    Err(Error::SerializationError(_)) => (),
                    Err(_) => break,
                }
                if claim_attempts >= 4 {
                    break;
                };
            }

            {
                let mut result = registry.lock().unwrap();
                result.push(Self {
                    idn,
                    claimed_resources: updated.ok(),
                    claim_attempts,
                });
            }
        }
    }

}
