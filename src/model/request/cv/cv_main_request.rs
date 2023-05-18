use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct CvMainRequest {
    /// 简历ID
    pub id: i64,
}