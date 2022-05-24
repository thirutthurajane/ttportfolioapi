use actix_web::{App, HttpRequest, HttpServer, web, middleware::Logger};
use crate::libs::db;
use crate::controllers::company_controller::{get_all_companies};

mod controllers;
mod libs;
mod models;
mod services;

#[actix_web::main]
async fn main()  -> std::io::Result<()> {

    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let conn = db::connect().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(get_all_companies)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}