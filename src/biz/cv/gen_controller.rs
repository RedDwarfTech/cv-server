use okapi::openapi3::OpenApi;
use rocket::{post, response::content};
use rocket_okapi::{settings::OpenApiSettings, openapi_get_routes_spec, openapi};
use rust_wheel::{model::user::login_user_info::LoginUserInfo, common::util::model_convert::box_rest_response};
use rocket::serde::json::Json;

use crate::{model::request::cv::gen_request::GenRequest, service::cv::gen_service::cv_gen_list};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add ]
}

/// # 查询简历生成记录
///
/// 查询简历生成记录
#[openapi(tag = "简历生成记录")]
#[post("/v1/list", data = "<request>")]
pub fn add(request: Json<GenRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = cv_gen_list(request.cv_name.clone(), &login_user_info);
    return box_rest_response(gen_cv);
}

