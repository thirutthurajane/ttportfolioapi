use actix_web::{HttpResponse, web, get, post, put, delete};
use mongodb::Database;
use crate::libs::err_handler::ApiError;
use crate::models::companies::FormCompany;
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

#[post("/api/companies")]
pub async fn add_company(conn: web::Data<Database>,form: web::Json<FormCompany>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(company_service::add_company(conn, form).await?)
    )
}

#[put("/api/companies/{id}")]
pub async fn update_company(conn: web::Data<Database>,id: web::Path<String>,
                            form: web::Json<FormCompany>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(company_service::update_company(conn, id.into_inner(), form).await?)
    )
}

#[delete("/api/companies/{id}")]
pub async fn delete_company(conn: web::Data<Database>, id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(company_service::delete_company(conn, id.into_inner()).await?)
    )
}