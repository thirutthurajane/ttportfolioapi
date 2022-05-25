use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpServer, web, middleware::Logger};
use actix_co
use clap::Parser;
use crate::libs::db;
use crate::controllers::company_controller::*;
use crate::controllers::work_controller::*;
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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(get_company)
            .service(get_all_companies)
            .service(update_company)
            .service(add_company)
            .service(delete_company)
            .service(add_work)
            .service(get_all_works)
            .service(get_work)
            .service(update_work)
            .service(delete_work)
            .service(get_work_with_comp)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
    })
        .bind(format!("{}:{}", &cfg.host, &cfg.port))?
        .run()
        .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}