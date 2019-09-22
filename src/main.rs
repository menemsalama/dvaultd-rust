extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate serde;
extern crate uuid;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod db;
mod models;
mod resources;
mod schema;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;

fn greet() -> impl actix_web::Responder {
    "hello world"
}

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = db::connect(database_url);
    let pool = db::pool(manager);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .route("/", actix_web::web::get().to(greet))
            .service(resources::users::resource())
    })
    // .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
}
