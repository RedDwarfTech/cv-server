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
    pub start: String,
    pub end: String,
    pub id: Option<i64>,
}