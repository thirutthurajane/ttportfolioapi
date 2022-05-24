use actix_web::web;
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