use crate::model::request::cv::gen_request::GenRequest;
use crate::service::cv::gen_service::{
    check_gen_status, check_paied_plan, cv_gen_list_render, cv_gen_page, del_gen_impl, get_cv_src,
    pick_task,
};
use crate::{
    model::request::cv::render_result_request::RenderResultRequest,
    service::cv::gen_service::{cv_gen_list, update_gen_result},
};
use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::common::wrapper::rocket_http_resp::{box_error_rest_response, box_rest_response};
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: list,
        flush_render_result,
        submit_gen_task,
        get_list_for_render,
        page,
        del_gen,
        pick_one_task,
        check_status,
        get_src
    ]
}

/// # 查询简历生成记录
///
/// 查询简历生成记录
#[openapi(tag = "简历渲染")]
#[get("/v1/list?<cv_name>")]
pub fn list(cv_name: Option<String>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = cv_gen_list(cv_name.clone(), &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 查询一个简历生成记录
///
/// 查询一个简历生成记录
#[openapi(tag = "简历渲染")]
#[get("/v1/pick")]
pub fn pick_one_task() -> content::RawJson<String> {
    let gen_cv = pick_task();
    match gen_cv {
        Ok(queue_cv) => {
            return box_rest_response(queue_cv.unwrap_or_default());
        }
        Err(e) => {
            return box_error_rest_response("", "PICK_TASK_FAILED".to_string(), e.to_string());
        }
    }
}

/// # 分页查询简历生成记录
///
/// 分页查询简历生成记录
#[openapi(tag = "简历渲染")]
#[get("/v1/page?<cv_name>")]
pub fn page(cv_name: Option<String>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = cv_gen_page(cv_name.clone(), &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 提交渲染任务
///
/// 提交渲染任务
#[openapi(tag = "简历渲染")]
#[post("/v1/submit", data = "<request>")]
pub fn submit_gen_task(
    request: Json<GenRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = check_paied_plan(&request, &login_user_info);
    match gen_cv {
        Ok(task) => {
            return box_rest_response(task);
        }
        Err(err) => {
            let source = err.to_string();
            return box_error_rest_response("", "submit failed".to_string(), source.to_string());
        }
    }
}

/// # 渲染器查询简历生成记录
///
/// 渲染器查询简历生成记录
#[openapi(tag = "简历渲染")]
#[get("/v1/render-list?<cv_name>")]
pub fn get_list_for_render(cv_name: Option<String>) -> content::RawJson<String> {
    let gen_cv = cv_gen_list_render(cv_name.clone());
    return box_rest_response(gen_cv);
}

/// # 更新简历生成结果
///
/// 更新简历生成结果
#[openapi(tag = "简历渲染")]
#[put("/v1/result", data = "<request>")]
pub fn flush_render_result(request: Json<RenderResultRequest>) -> content::RawJson<String> {
    let gen_cv = update_gen_result(request);
    return box_rest_response(gen_cv);
}

/// # 删除简历生成结果
///
/// 删除简历生成结果
#[openapi(tag = "简历渲染")]
#[delete("/v1/<id>")]
pub fn del_gen(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = del_gen_impl(&id, &login_user_info);
    if gen_cv {
        return box_rest_response(id);
    } else {
        return box_error_rest_response("-1", "500".to_string(), "failed".to_string());
    }
}

/// # 查看简历生成结果
///
/// 查看简历生成结果
#[openapi(tag = "简历渲染")]
#[get("/v1/status?<ids>")]
pub fn check_status(ids: String, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = check_gen_status(ids, &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 获取简历源码
///
/// 获取简历源码
#[openapi(tag = "获取简历源码")]
#[get("/v1/src?<id>")]
pub fn get_src(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = get_cv_src(id, &login_user_info);
    return box_rest_response(gen_cv);
}
