use crate::model::request::cv::gen_request::GenRequest;
use crate::service::cv::gen_service::{
    check_gen_status, cv_gen_list_render, cv_gen_page, del_gen_impl, pick_task, check_paied_plan,
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
        page,
        del_gen,
        pick_one_task,
        check_status
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

/// # 查询一个简历生成记录
///
/// 查询一个简历生成记录
#[openapi(tag = "查询一个简历生成记录")]
#[get("/v1/pick")]
pub fn pick_one_task() -> content::RawJson<String> {
    let gen_cv = pick_task();
    match gen_cv {
        Ok(gen_cv) => {
            return box_rest_response(gen_cv);
        }
        Err(e) => {
            return box_error_rest_response("", "error".to_string(), e.to_string());
        }
    }
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
    let gen_cv = check_paied_plan(&request, &login_user_info);
    match gen_cv {
        Ok(task) => {
            return box_rest_response(task);
        }
        Err(err) => {
            let source = err.source();
            return box_error_rest_response(
                "",
                "submit failed".to_string(),
                source.unwrap().to_string(),
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
pub fn flush_render_result(request: Json<RenderResultRequest>) -> content::RawJson<String> {
    let gen_cv = update_gen_result(request);
    return box_rest_response(gen_cv);
}

///
/// 删除简历生成结果
#[openapi(tag = "删除简历生成结果")]
#[delete("/v1/<id>")]
pub fn del_gen(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = del_gen_impl(&id, &login_user_info);
    if gen_cv {
        return box_rest_response(id);
    } else {
        return box_error_rest_response("-1", "500".to_string(), "failed".to_string());
    }
}

///
/// 查看简历生成结果
#[openapi(tag = "查看简历生成结果")]
#[get("/v1/status?<ids>")]
pub fn check_status(ids: String, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = check_gen_status(ids, &login_user_info);
    return box_rest_response(gen_cv);
}
