use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct EduRequest {
    pub cv_id: i64,
    pub edu_addr: String,
    pub degree: String,
    pub major: String,
    pub admission: String,
    pub graduation: String,
    pub city: Option<String>,
    pub id: Option<i64>,
}