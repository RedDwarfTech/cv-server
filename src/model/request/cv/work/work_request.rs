use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct WorkRequest {
    pub cv_id: i64,
    pub company: String,
    pub job: String,
    pub city: String,
    pub work_start: String,
    pub work_end: String,
    pub id: Option<i64>,
    pub duty: Option<String>,
}