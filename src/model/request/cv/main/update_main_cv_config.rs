use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

// https://stackoverflow.com/questions/72249171/rust-diesel-conditionally-update-fields
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct UpdateMainCvConfig {
    pub cv_id: i64,
    pub theme: Option<String>,
    pub font_size: Option<String>
}