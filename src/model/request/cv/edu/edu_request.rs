use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct EduRequest {
    /// 简历名称
    pub cv_name: Option<String>,
    pub cv_id: i64,
    pub edu_addr: String,
    pub id: Option<i64>,
}