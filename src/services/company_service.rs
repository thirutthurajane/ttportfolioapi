use actix_web::web;
use bson::doc;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::Database;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use crate::libs::err_handler::ApiError;
use crate::models::companies::{Company, FormCompany};

pub async fn get_all_companies(conn: web::Data<Database>) -> Result<Vec<Company>, ApiError> {
    let mut cursor = conn.collection::<Company>("companies").find(None, None).await?;
    let mut comps_result: Vec<Company> = Vec::new();
    while let Some(comp) = cursor.try_next().await?{
        let c = comp;
        comps_result.push(c);
    }
    Ok(
        comps_result
    )
}

pub async fn get_company(conn: web::Data<Database>, id: String) -> Result<Company, ApiError> {
    let result = conn.collection::<Company>("companies").find_one(doc! {
       "_id": ObjectId::parse_str(id)?
    },None).await?.unwrap();

    Ok(
        result
    )
}

pub async fn add_company(conn: web::Data<Database>,form: web::Json<FormCompany>) -> Result<InsertOneResult, ApiError> {
    Ok(
        conn.collection::<Company>("companies").insert_one(Company {
            _id: ObjectId::new(),
            comp_name: form.comp_name.clone(),
            comp_detail: form.comp_detail.clone(),
            position: form.position.clone(),
            work_period: form.work_period.clone()
        }, None).await?
    )
}

pub async fn update_company(conn: web::Data<Database>, id: String,form: web::Json<FormCompany>) -> Result<UpdateResult, ApiError> {

    let data = form.into_inner();
    Ok(
        conn.collection::<Company>("companies").update_one(doc! {
            "_id": ObjectId::parse_str(id)?
        },doc! {
            "$set": {
                "compName": data.comp_name.clone(),
                "compDetail": data.comp_detail.clone(),
                "position": data.position.clone(),
                "workPeriod": data.work_period.clone()
            }
        },None).await?
    )
}

pub async fn delete_company(conn: web::Data<Database>,id: String) -> Result<DeleteResult, ApiError> {
    Ok(
        conn.collection::<Company>("companies").delete_one(doc! {
             "_id": ObjectId::parse_str(id)?
        }, None).await?
    )
}