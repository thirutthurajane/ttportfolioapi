use actix_web::{App, HttpRequest, HttpServer, web, middleware::Logger};
use clap::Parser;
use crate::libs::db;
use crate::controllers::company_controller::{get_all_companies, get_company, add_company, update_company, delete_company};
use crate::models::config;

mod controllers;
mod libs;
mod models;
mod services;

#[actix_web::main]
async fn main()  -> std::io::Result<()> {

    dotenv::dotenv().ok();
    let cfg = config::Config::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let conn = db::connect().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(get_company)
            .service(get_all_companies)
            .service(update_company)
            .service(add_company)
            .service(delete_company)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind(format!("{}:{}", &cfg.host, &cfg.port))?
        .run()
        .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}