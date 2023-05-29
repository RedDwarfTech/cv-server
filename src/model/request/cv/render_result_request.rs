use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct RenderResultRequest {
    /// 渲染状态
    pub gen_status: i32,
    /// 渲染记录ID
    pub id: i64,
    pub path: String,
}