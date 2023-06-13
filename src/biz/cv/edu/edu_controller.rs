use crate::model::request::cv::edu::edu_request::EduRequest;
use crate::service::cv::edu::edu_service::{add_edu, del_edu_item, get_ui_edu_list};
use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket::{delete, get};
use rocket::{post, response::content};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::common::util::model_convert::box_error_rest_response;
use rust_wheel::{
    common::util::model_convert::box_rest_response, model::user::login_user_info::LoginUserInfo,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_or_update, edu_list, del_edu]
}

/// # 保存教育信息
///
/// 教育信息
#[openapi(tag = "简历-教育信息")]
#[post("/v1", data = "<request>")]
pub fn add_or_update(
    request: Json<EduRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let cv_edu_list = add_edu(&request, &login_user_info);
    match cv_edu_list {
        Ok(edu) => {
            return box_rest_response(edu);
        }
        Err(e) => {
            return box_error_rest_response("-1", "500".to_string(), e.to_string());
        }
    }
}

/// # 获取教育信息
///
/// 获取教育信息
#[openapi(tag = "简历-教育信息")]
#[get("/v1?<cv_id>")]
pub fn edu_list(cv_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = get_ui_edu_list(&cv_id, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 删除教育信息
///
/// 删除教育信息
#[openapi(tag = "简历-教育信息")]
#[delete("/v1/item?<edu_id>")]
pub fn del_edu(edu_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = del_edu_item(&edu_id, &login_user_info);
    if cv_edu_list {
        return box_rest_response(edu_id);
    } else {
        return box_error_rest_response("-1", "500".to_string(), "failed".to_string());
    }
}
