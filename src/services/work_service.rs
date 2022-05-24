use actix_web::web;
use bson::doc;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::Database;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use crate::libs::err_handler::ApiError;
use crate::models::works::{Works, FormWork, WorksWithCompany};

pub async fn get_all_works(conn: web::Data<Database>) -> Result<Vec<Works>, ApiError> {
    let mut cursor = conn.collection::<Works>("works").find(None, None).await?;
    let mut work_results: Vec<Works> = Vec::new();
    while let Some(work) = cursor.try_next().await?{
        work_results.push(work);
    }
    Ok(
        work_results
    )
}

pub async fn get_work(conn: web::Data<Database>, id: String) -> Result<Works, ApiError> {
    let result = conn.collection::<Works>("works").find_one(doc! {
       "_id": ObjectId::parse_str(id)?
    },None).await?.unwrap();

    Ok(
        result
    )
}

pub async fn get_work_with_comp(conn: web::Data<Database>, id: String) -> Result<Vec<WorksWithCompany>, ApiError> {
    let mut cursor = conn.collection::<Works>("works").aggregate(
        vec! [
            doc! {
                "$match": {
                    "_id": ObjectId::parse_str(id)?
                }
            },
            doc! {
                "$lookup": {
                    "from": "companies",
                    "localField": "compId",
                    "foreignField": "_id",
                    "as": "company"
                }
            },
            doc! {
                "$unwind": "$company",
            },
        ],None).await?;
    let mut work_results: Vec<WorksWithCompany> = Vec::new();
    while let Some(work) = cursor.try_next().await?{
        let res: WorksWithCompany = bson::from_document(work)?;
        work_results.push(res);
    }
    Ok(
        work_results
    )
}

pub async fn add_work(conn: web::Data<Database>,form: web::Json<FormWork>) -> Result<InsertOneResult, ApiError> {
    Ok(
        conn.collection::<Works>("works").insert_one(Works {
            _id: ObjectId::new(),
            comp_id: form.comp_id.clone(),
            project_name: form.project_name.clone(),
            detail: form.detail.clone()
        }, None).await?
    )
}

pub async fn update_work(conn: web::Data<Database>, id: String,form: web::Json<FormWork>) -> Result<UpdateResult, ApiError> {

    let data = form.into_inner();
    Ok(
        conn.collection::<Works>("works").update_one(doc! {
            "_id": ObjectId::parse_str(id)?
        },doc! {
            "$set": {
                "comp_id": data.comp_id.clone(),
                "project_name": data.project_name.clone(),
                "detail": data.detail.clone()
            }
        },None).await?
    )
}

pub async fn delete_work(conn: web::Data<Database>,id: String) -> Result<DeleteResult, ApiError> {
    Ok(
        conn.collection::<Works>("works").delete_one(doc! {
             "_id": ObjectId::parse_str(id)?
        }, None).await?
    )
}