use crate::{
    model::request::cv::main::edit_main_request::EditMainRequest,
    service::cv::cv_main_service::{
        cv_main_list, del_cv_by_id, get_cv_by_id, get_cv_summary, update_cv_main,
    },
};
use okapi::openapi3::OpenApi;
use rocket::{delete, get, post, response::content, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::{
    common::util::model_convert::{box_error_rest_response, box_rest_response},
    model::user::login_user_info::LoginUserInfo,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: get,
        get_cv_detail,
        edit_cv_summary,
        get_summary,
        del_cv
    ]
}

/// # 查询简历
///
/// 查询简历
#[openapi(tag = "用户简历列表")]
#[get("/v1/cv/list")]
pub fn get(login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let main_cvs = cv_main_list(&login_user_info);
    return box_rest_response(main_cvs);
}

/// # 根据ID查询简历
///
/// 根据ID查询简历
#[openapi(tag = "根据ID查询简历")]
#[get("/v1/cv/<id>")]
pub fn get_cv_detail(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = get_cv_by_id(id, &login_user_info);
    if let Some(v) = gen_cv {
        return box_rest_response(v);
    } else {
        return box_rest_response("no data");
    }
}

/// # 根据ID删除简历
///
/// 根据ID删除简历
#[openapi(tag = "根据ID删除简历")]
#[delete("/v1/cv/<id>")]
pub fn del_cv(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let del_result = del_cv_by_id(id, &login_user_info);
    if del_result {
        return box_rest_response(id);
    } else {
        return box_error_rest_response("-1", "500".to_string(), "failed".to_string());
    }
}

/// # 根据ID查询简历基础信息
///
/// 根据ID查询简历基础信息
#[openapi(tag = "根据ID查询简历基础信息")]
#[get("/v1/summary/<id>")]
pub fn get_summary(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = get_cv_summary(id, &login_user_info);
    if let Some(v) = gen_cv {
        return box_rest_response(v);
    } else {
        return box_rest_response("no data");
    }
}

/// # 更新简历基础信息
///
/// 更新简历基础信息
#[openapi(tag = "更新简历基础信息")]
#[post("/v1/cv", data = "<request>")]
pub fn edit_cv_summary(
    request: Json<EditMainRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_cv_main(&request, &login_user_info);
    return box_rest_response(gen_cv);
}
