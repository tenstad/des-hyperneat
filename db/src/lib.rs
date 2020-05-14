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
use mongodb::{
    options::{auth::Credential, ClientOptions, StreamAddress},
    sync::{Client, Collection, Database},
};
use serde::Serialize;

#[allow(dead_code)]
pub struct Mongo {
    client: Client,
    db: Database,
    collection: Collection,
}

#[allow(dead_code)]
pub struct Entry {
    collection: Collection,
    id_query: OrderedDocument,
    num_events: u64,
}

#[allow(dead_code)]
impl Mongo {
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
        let collection = db.collection(&DB.collection);

        Self {
            client,
            db,
            collection,
        }
    }

    pub fn entry<T: Serialize>(&mut self, entry: &T) -> Entry {
        let document = self.add_entry_data(entry);
        let id = self.insert(&document);

        Entry {
            collection: self.collection.clone(),
            id_query: doc! {
                "_id": id
            },
            num_events: 0,
        }
    }

    fn add_entry_data<T: Serialize>(&mut self, data: &T) -> OrderedDocument {
        if let Document(mut document) = to_bson(data).unwrap() {
            document.insert("entry_time", Utc::now());
            document
        } else {
            panic!("unable to serialize data");
        }
    }

    fn insert<T: Serialize>(&self, data: &T) -> ObjectId {
        if let Document(document) = to_bson(data).unwrap() {
            let result = self.collection.insert_one(document, None).unwrap();
            if let Bson::ObjectId(id) = result.inserted_id {
                id
            } else {
                panic!("unable to get id from insert")
            }
        } else {
            panic!("unable to serialize data");
        }
    }
}

#[allow(dead_code)]
impl Entry {
    pub fn push<T: Serialize>(&mut self, event: &T) {
        let document = self.add_event_data(event);
        let update = doc! {
            "$push": {"events": document}
        };
        self.collection
            .update_one(self.id_query.clone(), update, None)
            .unwrap();
    }

    fn add_event_data<T: Serialize>(&mut self, event: &T) -> OrderedDocument {
        if let Document(mut document) = to_bson(event).ok().unwrap() {
            document.insert("event_time", Utc::now());
            document.insert("event_id", self.num_events);
            self.num_events += 1;
            document
        } else {
            panic!("unable to serialize data");
        }
    }
}
