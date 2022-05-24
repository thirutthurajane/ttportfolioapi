use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use crate::models::companies::Company;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Works {
    pub _id: ObjectId,
    #[serde(rename = "compId")]
    pub comp_id: ObjectId,
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub detail: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormWork {
    #[serde(rename = "compId")]
    pub comp_id: ObjectId,
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub detail: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorksWithCompany {
    pub _id: ObjectId,
    #[serde(rename = "compId")]
    pub comp_id: ObjectId,
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub detail: String,
    pub company: Company
}