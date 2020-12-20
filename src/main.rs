#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files::{Files};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod routes;
mod models;
mod schema;
mod url_shortener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let host = std::env::var("HOST").expect("HOST");
    let port = std::env::var("PORT").expect("PORT");
    let bind = format!("{}:{}", host, port);
    println!("Starting server at: http://{}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(
                web::resource("/").route(web::get().to(routes::index))
            )
            .service(
                Files::new("/static/", "./static/").use_last_modified(true)
            )
            .service(
                web::resource("/{code}").route(web::get().to(routes::redirect))
            )
            .service(
                web::resource("/api/v1/generate").route(web::post().to(routes::create_code))
            )
    })
    .bind(&bind)?
    .run()
    .await
}
