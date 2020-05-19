#[macro_use(doc)]
extern crate bson;
extern crate mongodb;

#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

mod conf;

use bson::{
    oid::ObjectId,
    ordered::OrderedDocument,
    to_bson,
    Bson::{self, Document},
};
use chrono::offset::Utc;
use conf::DB;
use hex;
use mongodb::{
    options::{auth::Credential, ClientOptions, StreamAddress},
    sync::{Client, Database},
};
use serde::Serialize;
use std::env;
use std::{thread, time};

#[allow(dead_code)]
pub struct Mongo {
    client: Client,
    db: Database,
}

#[allow(dead_code)]
pub struct Entry {
    mongo: Mongo,
    id_query: OrderedDocument,
}

#[allow(dead_code)]
impl Mongo {
    /// Creates a dabase instance that can yeield entry instances
    pub fn new() -> Self {
        let options = ClientOptions::builder()
            .hosts(vec![StreamAddress {
                hostname: DB.host.clone(),
                port: Some(27017),
            }])
            .credential(
                Credential::builder()
                    .username(DB.username.clone())
                    .password(DB.password.clone())
                    .build(),
            )
            .build();

        let client = Client::with_options(options).unwrap();
        let db = client.database(&DB.database);

        Self { client, db }
    }

    pub fn reconnect(&mut self) {
        println!("Reconnecting...");
        let new = Mongo::new();
        self.client = new.client;
        self.db = new.db;
    }

    /// Returns a job instance entry, able to
    /// push future events in this job instance.
    pub fn entry<T: Serialize>(&mut self, data: &T) -> Entry {
        let job_id = if let Some(job_id) = self.add_details_to_job(data) {
            job_id
        } else {
            str_to_id(&DB.job_id)
        };
        let entry_id = self.create_log_entry(job_id);

        Entry {
            mongo: Self::new(),
            id_query: doc! {
                "_id": entry_id
            },
        }
    }

    /// Update the job description with the exect config,
    /// if not already done by another instance of the job.
    /// If this is a standalone run, create new job for it.
    fn add_details_to_job<T: Serialize>(&mut self, data: &T) -> Option<ObjectId> {
        if let Document(document) = to_bson(data).unwrap() {
            if DB.job_id == "0" {
                Some(self.crate_new_single_run_job(document))
            } else {
                self.add_details_to_existing_job(document);
                None
            }
        } else {
            panic!("unable to serialize data");
        }
    }

    fn crate_new_single_run_job(&mut self, document: OrderedDocument) -> ObjectId {
        let document = doc! {
            "single": true,
            "start_time": Utc::now(),
            "config": document,
        };

        loop_insert(self, &DB.job_collection, document)
    }

    fn add_details_to_existing_job(&mut self, document: OrderedDocument) {
        let query = doc! {
            "_id": str_to_id(&DB.job_id),
            "config": {"$exists": false},
        };
        let update = doc! {
            "$set": {"config": document},
        };

        loop_update(self, &DB.job_collection, query, update);
    }

    /// Creates a new log entry for this job instance, returns id.
    fn create_log_entry(&mut self, job_id: ObjectId) -> ObjectId {
        let document = doc! {
            "job_id": job_id,
            "start_time": Utc::now(),
            "node_name": env::var("HOSTNAME").unwrap_or("".to_owned()),
        };

        loop_insert(self, &DB.log_collection, document)
    }
}

#[allow(dead_code)]
impl Entry {
    pub fn push<T: Serialize>(&mut self, event: &T, iteration: u64) {
        let document = self.add_event_data(event, iteration);
        let update = doc! {
            "$push": {"events": document}
        };
        loop_update(
            &mut self.mongo,
            &DB.log_collection,
            self.id_query.clone(),
            update,
        );
    }

    fn add_event_data<T: Serialize>(&mut self, event: &T, iteration: u64) -> OrderedDocument {
        if let Document(mut document) = to_bson(event).ok().unwrap() {
            document.insert("event_time", Utc::now());
            document.insert("iteration", iteration);
            document
        } else {
            panic!("unable to serialize data");
        }
    }
}

fn str_to_id(string: &String) -> ObjectId {
    let bytes: Vec<u8> = hex::decode(string.as_bytes()).unwrap();
    let mut byte_array: [u8; 12] = [0; 12];
    byte_array[..].copy_from_slice(&bytes[..]);
    ObjectId::with_bytes(byte_array)
}

fn loop_insert(mongo: &mut Mongo, collection: &String, document: OrderedDocument) -> ObjectId {
    let mut sleep_time = 5;
    loop {
        match mongo
            .db
            .collection(collection)
            .insert_one(document.clone(), None)
        {
            Ok(result) => {
                if let Bson::ObjectId(id) = result.inserted_id {
                    return id;
                } else {
                    panic!("unable to get inserted entry id");
                }
            }
            Err(error) => {
                println!("Connection error: {:#?}", error);

                thread::sleep(time::Duration::from_secs(sleep_time));
                sleep_time = (2 * sleep_time).min(60);
                mongo.reconnect();
            }
        }
    }
}

fn loop_update(
    mongo: &mut Mongo,
    collection: &String,
    query: OrderedDocument,
    update: OrderedDocument,
) {
    let mut sleep_time = 5;
    loop {
        match mongo
            .db
            .collection(collection)
            .update_one(query.clone(), update.clone(), None)
        {
            Ok(_) => {
                break;
            }
            Err(error) => {
                println!("Connection error: {:#?}", error);

                thread::sleep(time::Duration::from_secs(sleep_time));
                sleep_time = (2 * sleep_time).min(60);
                mongo.reconnect();
            }
        }
    }
}
