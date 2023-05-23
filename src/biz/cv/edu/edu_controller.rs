use crate::model::request::cv::edu::edu_request::EduRequest;
use crate::service::cv::edu::edu_service::{add_edu, get_edu_list, del_edu_item};
use okapi::openapi3::OpenApi;
use rocket::{get, delete};
use rocket::serde::json::Json;
use rocket::{post, response::content};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::{
    common::util::model_convert::box_rest_response, model::user::login_user_info::LoginUserInfo,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add, edu_list, del_edu]
}

/// # 保存教育信息
///
/// 教育信息
#[openapi(tag = "教育信息")]
#[post("/v1", data = "<request>")]
pub fn add(request: Json<EduRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = add_edu(&request, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 获取教育信息
///
/// 获取教育信息
#[openapi(tag = "教育信息")]
#[get("/v1?<cv_id>")]
pub fn edu_list(cv_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = get_edu_list(&cv_id, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 删除教育信息
///
/// 删除教育信息
#[openapi(tag = "删除教育信息")]
#[delete("/v1?<edu_id>")]
pub fn del_edu(edu_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = del_edu_item(&edu_id, &login_user_info);
    return box_rest_response(cv_edu_list);
}