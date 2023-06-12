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
    pub job: String,
    pub workplace: String,
    pub cv_name: String,
    pub stackoverflow: Option<String>,
    pub github: Option<String>,
    pub blog: Option<String>,
    pub remark: Option<String>,
}