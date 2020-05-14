#[macro_use(bson, doc)]
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
use chrono::{offset::Utc, DateTime};
use conf::DB;
use mongodb::{
    options::{auth::Credential, ClientOptions, StreamAddress},
    sync::{Client, Collection, Database},
};
use serde::Serialize;

struct MongoDB {
    client: Client,
    db: Database,
    collection: Collection,
}

struct Entry {
    data: EntryData,
    collection: Collection,
    id_query: OrderedDocument,
    id: ObjectId,
    num_events: u64,
}

#[derive(Serialize)]
struct EntryData {
    name: String,
    start_time: DateTime<Utc>,
    events: Vec<()>,
}

impl MongoDB {
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

    pub fn entry(&mut self, name: String) -> Entry {
        let data = EntryData {
            name,
            start_time: Utc::now(),
            events: Vec::new(),
        };
        let id = self.insert(&data);

        Entry {
            data,
            collection: self.collection.clone(),
            id: id.clone(),
            id_query: doc! {
                "_id": id
            },
            num_events: 0,
        }
    }

    fn insert<T: Serialize>(&self, data: &T) -> ObjectId {
        if let Document(document) = to_bson(data).ok().unwrap() {
            let result = self.collection.insert_one(document, None).ok().unwrap();
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

impl Entry {
    pub fn push<T: Serialize>(&mut self, data: &T) {
        let document = self.add_event_data(data);
        let update = doc! {
            "$push": {"events": document}
        };
        self.collection
            .update_one(self.id_query.clone(), update, None)
            .unwrap();
    }

    fn add_event_data<T: Serialize>(&mut self, data: &T) -> OrderedDocument {
        if let Document(mut document) = to_bson(data).ok().unwrap() {
            document.insert("event_time", Utc::now());
            document.insert("event_id", self.num_events);
            self.num_events += 1;
            document
        } else {
            panic!("unable to serialize data");
        }
    }
}
