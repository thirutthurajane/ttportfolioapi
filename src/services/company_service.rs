use actix_web::web;
use bson::doc;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::Database;
use crate::libs::err_handler::ApiError;
use crate::models::companies::Company;

pub async fn get_all_companies(conn: web::Data<Database>) -> Result<Vec<Company>, ApiError> {
    let mut cursor = conn.collection::<Company>("companies").find(None, None).await?;
    let mut compsResult: Vec<Company> = Vec::new();
    while let Some(comp) = cursor.try_next().await?{
        let c = comp;
        compsResult.push(c);
    }
    Ok(
        compsResult
    )
}

pub async fn get_company(conn: web::Data<Database>, id: String) -> Result<Option<Company>, ApiError> {
    let result = conn.collection::<Company>("companies").find_one(doc! {
       "_id": ObjectId::parse_str(id)?
    },None).await?;

    Ok(
        result
    )
}