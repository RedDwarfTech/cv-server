use crate::model::request::cv::skills::skills_request::SkillsRequest;
use crate::service::cv::skills::skills_exp_service::{add_skill, get_ui_skill_list, del_skill_item};
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
    openapi_get_routes_spec![settings: add, skills_list, del_skill]
}

/// # 保存专业技能
///
/// 专业技能
#[openapi(tag = "简历-专业技能")]
#[post("/v1", data = "<request>")]
pub fn add(request: Json<SkillsRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = add_skill(&request, &login_user_info);
    match cv_edu_list {
        Ok(work) => {
            return box_rest_response(work);
        }
        Err(e) => {
            return box_error_rest_response("-1", "500".to_string(), e.to_string());
        }
    }
}

/// # 获取简历专业技能列表
///
/// 获取简历专业技能列表
#[openapi(tag = "简历-专业技能")]
#[get("/v1?<cv_id>")]
pub fn skills_list(cv_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = get_ui_skill_list(&cv_id, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 删除专业技能
///
/// 删除专业技能
#[openapi(tag = "简历-专业技能")]
#[delete("/v1/item?<skill_id>")]
pub fn del_skill(skill_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = del_skill_item(&skill_id, &login_user_info);
    if cv_edu_list {
        return box_rest_response(skill_id);
    }else{
        return box_error_rest_response("-1","500".to_string(),"failed".to_string());
    }
}