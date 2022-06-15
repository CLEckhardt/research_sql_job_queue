use crate::store::{Entry, NewEntry, PgStore};
use diesel::result::{DatabaseErrorKind, Error};
#[allow(dead_code, unused_must_use, unused_imports, unused_variables)]
use log::debug;
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
                updated = store.execute_attempt(&idn);
                claim_attempts += 1;
                debug!("{:?}", &updated);
                match &updated {
                    Ok(v) => {
                        // Break if claim was successful
                        if v.len() > 0 {
                            break;
                        };
                    }
                    // Retry on serialization error
                    Err(Error::DatabaseError(DatabaseErrorKind::SerializationFailure, _)) => {
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    }
                    Err(_) => break,
                }
                if claim_attempts >= 8 {
                    break;
                };
            }

            /*
            while updated.is_err() {
                if claim_attempts >= 10 {
                    break;
                };
                match updated {
                    // Retry on serialization error
                    Err(Error::SerializationError(_)) => {
                        updated = store.execute_attempt(&idn);
                        claim_attempts += 1;
                    }
                    _ => break,
                };
            }*/

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

    /*
    pub async fn attempt_claim(mut self) {
        use crate::schema::queue::dsl::*;

        // Attempt to claim a resource

        let mut updated = store.execute_attempt(&self.id);
        self.claim_attempts += 1;

        while updated.is_err() {
            if &self.claim_attempts >= &10 {
                break;
            };
            match updated {
                // Retry on serialization error
                Err(Error::SerializationError(_)) => {
                    updated = store.execute_attempt(&self.id);
                    self.claim_attempts += 1;
                }
                _ => break,
            };
        }

        self.claimed_resources = updated.ok();

        {
            let mut res = result.lock().unwrap();
            res.push(self);
        }

        //println!("Result: {:?}", updated);
    }*/
}
