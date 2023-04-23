use crate::{models::event_model::Event, repository::mongodb_repository::MongoRepo};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CreateEventResponse {
    id: String,
    ok: bool,
}

#[post("/")]
pub async fn create_event(db: Data<MongoRepo>, new_event: Json<Event>) -> HttpResponse {
    let data = Event {
        id: None,
        kind: new_event.kind.to_owned(),
        title: new_event.title.to_owned(),
        date: new_event.date.to_owned(),
        location: new_event.location.to_owned(),
        description: new_event.description.to_owned(),
        image: new_event.image.to_owned(),
    };

    let event_detail = db.create_event(data).await;

    match event_detail {
        Ok(event) => {
            let id = event.inserted_id.as_object_id().unwrap().to_string();
            let response = CreateEventResponse { id, ok: true };
            let json_response = serde_json::to_string_pretty(&response).unwrap();

            HttpResponse::Ok()
                .content_type("application/json")
                .body(json_response)
        }

        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_event(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_event(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
