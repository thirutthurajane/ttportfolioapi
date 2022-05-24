use actix_web::{HttpResponse, web, get, post, put, delete};
use mongodb::Database;
use crate::libs::err_handler::ApiError;
use crate::models::works::FormWork;
use crate::services::work_service;

#[get("/api/works")]
pub async fn get_all_works(conn: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(work_service::get_all_works(conn).await?)
    )
}

#[get("/api/works/{id}")]
pub async fn get_work(conn: web::Data<Database>,id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(work_service::get_work(conn, id.into_inner()).await?)
    )
}

#[get("/api/works/{id}/company")]
pub async fn get_work_with_comp(conn: web::Data<Database>,id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(work_service::get_work_with_comp(conn, id.into_inner()).await?)
    )
}

#[post("/api/works")]
pub async fn add_work(conn: web::Data<Database>,form: web::Json<FormWork>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(work_service::add_work(conn, form).await?)
    )
}

#[put("/api/works/{id}")]
pub async fn update_work(conn: web::Data<Database>,id: web::Path<String>,
                            form: web::Json<FormWork>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(work_service::update_work(conn, id.into_inner(), form).await?)
    )
}

#[delete("/api/works/{id}")]
pub async fn delete_work(conn: web::Data<Database>, id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().json(work_service::delete_work(conn, id.into_inner()).await?)
    )
}