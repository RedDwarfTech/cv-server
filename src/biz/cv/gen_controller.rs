use crate::model::request::cv::gen_request::GenRequest;
use crate::service::cv::gen_service::{create_gen_task, cv_gen_list_render, cv_gen_page};
use crate::{
    model::request::cv::render_result_request::RenderResultRequest,
    service::cv::gen_service::{cv_gen_list, update_gen_result},
};
use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket::{get, post, put};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::common::util::model_convert::box_error_rest_response;
use rust_wheel::{
    common::util::model_convert::box_rest_response, model::user::login_user_info::LoginUserInfo,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: list,
        flush_render_result,
        submit_gen_task,
        get_list_for_render,
        page
    ]
}

/// # 查询简历生成记录
///
/// 查询简历生成记录
#[openapi(tag = "简历生成记录")]
#[get("/v1/list?<cv_name>")]
pub fn list(cv_name: Option<String>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = cv_gen_list(cv_name.clone(), &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 分页查询简历生成记录
///
/// 分页查询简历生成记录
#[openapi(tag = "简历生成记录分页")]
#[get("/v1/page?<cv_name>")]
pub fn page(cv_name: Option<String>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = cv_gen_page(cv_name.clone(), &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 提交渲染任务
///
/// 提交渲染任务
#[openapi(tag = "提交渲染任务")]
#[post("/v1/submit", data = "<request>")]
pub fn submit_gen_task(
    request: Json<GenRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = create_gen_task(&request, &login_user_info);
    match gen_cv {
        Ok(task) => {
            return box_rest_response(task);
        }
        Err(_) => {
            return box_error_rest_response(
                "error",
                "sub_failed".to_string(),
                "create task failed".to_string(),
            );
        }
    }
}

/// # 渲染器查询简历生成记录
///
/// 渲染器查询简历生成记录
#[openapi(tag = "简历生成记录(渲染器)")]
#[get("/v1/render-list?<cv_name>")]
pub fn get_list_for_render(cv_name: Option<String>) -> content::RawJson<String> {
    let gen_cv = cv_gen_list_render(cv_name.clone());
    return box_rest_response(gen_cv);
}

///
/// 更新简历生成结果
#[openapi(tag = "更新简历生成结果")]
#[put("/v1/result", data = "<request>")]
pub fn flush_render_result(
    request: Json<RenderResultRequest>
) -> content::RawJson<String> {
    let gen_cv = update_gen_result(request);
    return box_rest_response(gen_cv);
}
