use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct UpdateMainCvColor {
    pub cv_id: i64,
    pub main_color: String,
}