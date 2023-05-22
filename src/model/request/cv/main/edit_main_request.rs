use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct EditMainRequest {
    pub id: Option<i64>,
    pub employee_name: String,
    pub phone: String,
    pub email: String,
    pub birthday: String,
    pub job: String
}