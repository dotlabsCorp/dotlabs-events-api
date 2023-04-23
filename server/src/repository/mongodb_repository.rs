use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    Client, Collection,
};

use crate::models::event_model::Event;
pub struct MongoRepo {
    collection: Collection<Event>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_DB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable(mongo_db_uri)"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("dotlabs_events");
        let collection: Collection<Event> = db.collection("Event");
        MongoRepo { collection }
    }

    pub async fn create_event(&self, new_event: Event) -> Result<InsertOneResult, Error> {
        let new_doc = Event {
            id: None,
            kind: new_event.kind,
            title: new_event.title,
            date: new_event.date,
            location: new_event.location,
            description: new_event.description,
            image: new_event.image,
        };

        let event = self
            .collection
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating event");

        Ok(event)
    }

    pub async fn get_event(&self, id: &String) -> Result<Event, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .collection
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting event's detail");
        Ok(user_detail.unwrap())
    }
}
