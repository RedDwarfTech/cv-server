use okapi::openapi3::OpenApi;
use rocket::{put, get};
use rocket::serde::json::Json;
use rocket::{post, response::content};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::{
    common::util::model_convert::box_rest_response, model::user::login_user_info::LoginUserInfo,
};

use crate::{
    model::request::cv::{gen_request::GenRequest, render_result_request::RenderResultRequest},
    service::cv::gen_service::{cv_gen_list, update_gen_result},
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add, flush_render_result]
}

/// # 查询简历生成记录
///
/// 查询简历生成记录
#[openapi(tag = "简历生成记录")]
#[get("/v1/list?<cv_name>")]
pub fn add(cv_name: Option<String>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = cv_gen_list(cv_name.clone(), &login_user_info);
    return box_rest_response(gen_cv);
}

///
/// 更新简历生成结果
#[openapi(tag = "更新简历生成结果")]
#[put("/v1/result", data = "<request>")]
pub fn flush_render_result(
    request: Json<RenderResultRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_gen_result(request, &login_user_info);
    return box_rest_response(gen_cv);
}
