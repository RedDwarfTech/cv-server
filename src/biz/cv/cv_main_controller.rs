use okapi::openapi3::OpenApi;
use rocket::{post, response::content, get};
use rocket_okapi::{settings::OpenApiSettings, openapi_get_routes_spec, openapi};
use rust_wheel::{model::user::login_user_info::LoginUserInfo, common::util::model_convert::box_rest_response};

use crate::{ service::cv::{gen_service::cv_gen_list, cv_main_service::cv_main_list}};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get ]
}

/// # 查询简历
///
/// 查询简历
#[openapi(tag = "简历")]
#[get("/v1/cv")]
pub fn get(login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = cv_main_list(&login_user_info);
    return box_rest_response(gen_cv);
}

