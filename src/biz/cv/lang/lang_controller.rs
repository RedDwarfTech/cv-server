use crate::model::request::cv::lang::langs_request::LangsRequest;
use crate::service::cv::lang::lang_service::{add_lang, get_ui_lang_list, del_lang_item};
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
    openapi_get_routes_spec![settings: add, langs_list, del_lang]
}

/// # 保存专业技能
///
/// 专业技能
#[openapi(tag = "简历-语言技能")]
#[post("/v1", data = "<request>")]
pub fn add(
    request: Json<LangsRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let cv_edu_list = add_lang(&request, &login_user_info);
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
#[openapi(tag = "简历-语言技能")]
#[get("/v1?<cv_id>")]
pub fn langs_list(cv_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = get_ui_lang_list(&cv_id, &login_user_info);
    return box_rest_response(cv_edu_list);
}

/// # 删除专业技能
///
/// 删除专业技能
#[openapi(tag = "简历-语言技能")]
#[delete("/v1/item?<lang_id>")]
pub fn del_lang(lang_id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let cv_edu_list = del_lang_item(&lang_id, &login_user_info);
    if cv_edu_list {
        return box_rest_response(lang_id);
    } else {
        return box_error_rest_response("-1", "500".to_string(), "failed".to_string());
    }
}
