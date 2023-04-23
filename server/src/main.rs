use std::env;
extern crate dotenv;
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer};
use api::event_api::{create_event, get_event};
use dotenv::dotenv;
use repository::mongodb_repository::MongoRepo;

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let api_key = get_env_var("API_KEY").unwrap().unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(
                web::scope("/event")
                    .guard(guard::Header("X-Api-Key", api_key))
                    .service(create_event)
                    .service(get_event),
            )
            .default_service(web::route().to(HttpResponse::Forbidden))
    })
    .bind((get_host(), get_port()))?;

    println!("Server is running on http://{}:{}", get_host(), get_port());

    server.run().await
}

fn get_port() -> u16 {
    match get_env_var("PORT") {
        Ok(Some(val)) => val.parse().unwrap_or(8080),
        _ => 8080,
    }
}

fn get_host() -> String {
    match get_env_var("HOST") {
        Ok(Some(val)) => val.to_string(),
        _ => "127.0.0.1".to_string(),
    }
}

fn get_env_var(name: &str) -> Result<Option<&'static str>, env::VarError> {
    dotenv().ok();
    match env::var(name) {
        Ok(val) => Ok(Some(Box::leak(val.into_boxed_str()) as &'static str)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(err) => Err(err),
    }
}
