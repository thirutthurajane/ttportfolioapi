use actix_web::{HttpResponse, web, get, post};
use mongodb::Database;
use crate::libs::err_handler::ApiError;
use crate::services::company_service;

#[get("/api/companies")]
pub async fn get_all_companies(conn: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(company_service::get_all_companies(conn).await?)
    )
}

#[get("/api/companies/{id}")]
pub async fn get_company(conn: web::Data<Database>,id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(company_service::get_company(conn, id.into_inner()).await?)
    )
}