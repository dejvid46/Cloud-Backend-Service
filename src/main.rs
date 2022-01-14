// dependencies
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{error, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use r2d2_sqlite::{self, SqliteConnectionManager};
use std::env;
use std::path::PathBuf;

// modules
mod handlers;
mod jwt;
mod middleware;
mod models;
mod reserr;
mod utils;

mod db;
use db::{create_tables, Pool};
use reserr::ResErr;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let manager = SqliteConnectionManager::file("data.db");
    let pool = Pool::new(manager).unwrap();

    create_tables(&pool);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
            .data(pool.clone())
            // admin utils
            .route("/users", web::get().to(handlers::admin::get_users))
            .route(
                "/users/{id}",
                web::get().to(handlers::admin::get_user_by_id),
            )
            .route(
                "/users/{id}",
                web::delete().to(handlers::admin::delete_user),
            )
            .route("/users/{id}", web::patch().to(handlers::admin::update_user))
            .route("/users", web::post().to(handlers::admin::add_user))
            // user utils
            .route("/user", web::get().to(handlers::user::get_me))
            .route("/user", web::patch().to(handlers::user::update_me))
            // Login
            .route("/login", web::post().to(handlers::login::login))
            .route("/check_login", web::post().to(handlers::login::check_login))
            // cloud utils
            .route(
                "/file/{filename:.*}",
                web::get().to(handlers::file::get_file),
            )
            .route(
                "/file/{filename:.*}",
                web::post().to(handlers::file::post_file),
            )
            .route(
                "/file/{filename:.*}",
                web::delete().to(handlers::file::delete_file),
            )
            // folder utils
            .route(
                "/folder/{filename:.*}",
                web::get().to(handlers::folder::get_folder),
            )
            .route(
                "/folder/{filename:.*}",
                web::post().to(handlers::folder::create_folder),
            )
            .route(
                "/folder/{filename:.*}",
                web::delete().to(handlers::folder::delete_folder),
            )
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "./static"))
            .default_service(web::route().to(err404))
    })
    .bind(env::var("ADDRESS").unwrap())?
    .run()
    .await
}

async fn index(_req: HttpRequest) -> Result<NamedFile, ResErr> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    let file = NamedFile::open(path).map_err(|_| ResErr::BadClientData("error 404"))?;

    Ok(file)
}

async fn err404() -> Result<HttpResponse, ResErr> {
    Err(ResErr::BadClientData("error 404"))
}
