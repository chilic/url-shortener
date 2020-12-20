use actix_web::{web, http, Error, HttpRequest, HttpResponse};
use actix_files::{NamedFile};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// use crate::models::Url;
use crate::url_shortener::{encode, decode};
use crate::models::NewUrl;
use crate::actions;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}

pub async fn index(_req: HttpRequest) -> Result<NamedFile, Error> {
    Ok(NamedFile::open("static/index.html")?)
}

pub async fn redirect(pool: web::Data<DbPool>, code: web::Path<String>) -> Result<HttpResponse, Error> {
    let url_id = decode(code.to_owned());
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let url_item = web::block(move || actions::find_url_by_id(url_id as i32, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(url_item) = url_item {
        Ok(redirect_to(&url_item.url))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No url found with code: {}", code));
        Ok(res)
    }
}

pub async fn create_code(pool: web::Data<DbPool>, form: web::Json<NewUrl>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let url = web::block(move || actions::insert_new_url(&form.url, &conn))
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let code = encode(url.id as usize);

    Ok(HttpResponse::Ok().json(code))
}
