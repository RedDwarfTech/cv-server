use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket::{post, response::content};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::{
    common::util::model_convert::box_rest_response, model::user::login_user_info::LoginUserInfo,
};
use crate::model::request::cv::edu::edu_request::EduRequest;
use crate::service::cv::edu::edu_service::add_edu;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add]
}

/// # 保存学历信息
///
/// 学历信息
#[openapi(tag = "学历信息")]
#[post("/v1", data = "<request>")]
pub fn add(request: Json<EduRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = add_edu(&request, &login_user_info);
    return box_rest_response(cv_edu_list);
}