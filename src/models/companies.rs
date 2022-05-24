use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Company {
    pub _id: ObjectId,
    #[serde(rename = "compName")]
    pub comp_name: String,
    #[serde(rename = "compDetail")]
    pub comp_detail: String,
    pub position: String,
    #[serde(rename = "workPeriod")]
    pub work_period: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormCompany {
    #[serde(rename = "compName")]
    pub comp_name: String,
    #[serde(rename = "compDetail")]
    pub comp_detail: String,
    pub position: String,
    #[serde(rename = "workPeriod")]
    pub work_period: String
}