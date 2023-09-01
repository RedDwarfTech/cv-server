use crate::model::request::cv::project::project_request::ProjectRequest;
use crate::service::cv::project::project_exp_service::{
    add_project, del_project_item, get_ui_project_list,
};
use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket::{delete, get};
use rocket::{post, response::content};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::common::wrapper::rocket_http_resp::{box_error_rest_response, box_rest_response};
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add, project_list, del_project_exp]
}

/// # 保存项目经验
///
/// 项目经验
#[openapi(tag = "简历-项目经历")]
#[post("/v1", data = "<request>")]
pub fn add(
    request: Json<ProjectRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let cv_edu_list = add_project(&request, &login_user_info);
    match cv_edu_list {
        Ok(work) => {
            return box_rest_response(work);
        }
        Err(e) => {
            return box_error_rest_response("-1", "500".to_string(), e.to_string());
        }
    }
}

/// # 获取简历项目经验列表
///
/// 获取简历项目经验列表
#[openapi(tag = "简历-项目经历")]
#[get("/v1?<cv_id>")]
pub fn project_list(cv_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = get_ui_project_list(&cv_id, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 删除项目经验
///
/// 删除项目经验
#[openapi(tag = "简历-项目经历")]
#[delete("/v1/item?<work_id>")]
pub fn del_project_exp(work_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = del_project_item(&work_id, &login_user_info);
    if cv_edu_list {
        return box_rest_response(work_id);
    } else {
        return box_error_rest_response("-1", "500".to_string(), "failed".to_string());
    }
}
