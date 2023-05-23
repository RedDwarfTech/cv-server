use crate::model::request::cv::edu::edu_request::EduRequest;
use crate::service::cv::work::work_exp_service::{add_work, get_work_list, del_work_item};
use okapi::openapi3::OpenApi;
use rocket::{get, delete};
use rocket::serde::json::Json;
use rocket::{post, response::content};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::common::util::model_convert::box_error_rest_response;
use rust_wheel::{
    common::util::model_convert::box_rest_response, model::user::login_user_info::LoginUserInfo,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add, work_list, del_work]
}

/// # 保存工作经验
///
/// 工作经验
#[openapi(tag = "工作经验")]
#[post("/v1", data = "<request>")]
pub fn add(request: Json<EduRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = add_work(&request, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 获取工作经验
///
/// 获取工作经验
#[openapi(tag = "工作经验")]
#[get("/v1?<cv_id>")]
pub fn work_list(cv_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = get_work_list(&cv_id, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 删除工作经验
///
/// 删除工作经验
#[openapi(tag = "删除工作经验")]
#[delete("/v1/item?<edu_id>")]
pub fn del_work(edu_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = del_work_item(&edu_id, &login_user_info);
    if cv_edu_list {
        return box_rest_response(edu_id);
    }else{
        return box_error_rest_response("-1","500".to_string(),"failed".to_string());
    }
}