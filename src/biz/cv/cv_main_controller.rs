use okapi::openapi3::OpenApi;
use rocket::{get, response::content};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::{
    common::util::model_convert::box_rest_response, model::user::login_user_info::LoginUserInfo,
};

use crate::service::cv::cv_main_service::cv_main_list;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get]
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
