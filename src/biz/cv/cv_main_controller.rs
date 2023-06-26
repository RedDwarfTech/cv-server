use crate::{
    model::request::cv::main::{
        copy_main_cv::CopyMainCv, edit_main_request::EditMainRequest, edit_main_sort::EditMainSort,
        update_main_cv_color::UpdateMainCvColor, update_main_cv_theme::UpdateMainCvConfig,
        update_main_cv_tpl::UpdateMainCvTpl,
    },
    service::cv::cv_main_service::{
        copy_cv_main, cv_main_list, del_cv_by_id, get_cv_by_id, get_cv_summary,
        get_render_cv_by_id, update_cv_main, update_cv_main_color, update_cv_main_sort,
        update_cv_main_config, update_cv_template,
    },
};
use okapi::openapi3::OpenApi;
use rocket::{delete, get, post, put, response::content, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::{
    common::util::model_convert::{box_error_rest_response, box_rest_response},
    model::user::login_user_info::LoginUserInfo,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: get_cv_list,
        get_cv_detail,
        edit_cv_summary,
        get_summary,
        del_cv,
        get_render_cv_detail,
        edit_cv_sort,
        copy_cv,
        update_cv_tpl,
        update_cv_color,
        update_cv_theme,
        update_cv_config
    ]
}

/// # 查询用户简历列表
///
/// 查询用户简历列表
#[openapi(tag = "简历")]
#[get("/v1/cv/list")]
pub fn get_cv_list(login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let main_cvs = cv_main_list(&login_user_info);
    return box_rest_response(main_cvs);
}

/// # 根据ID查询简历
///
/// 根据ID查询简历
#[openapi(tag = "简历")]
#[get("/v1/cv/<id>")]
pub fn get_cv_detail(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let gen_cv = get_cv_by_id(id, &login_user_info);
    if let Some(v) = gen_cv {
        return box_rest_response(v);
    } else {
        return box_rest_response("no data");
    }
}

/// # 根据ID查询简历(渲染器)
///
/// 根据ID查询简历(渲染器)
#[openapi(tag = "简历")]
#[get("/v1/render-cv/<id>")]
pub fn get_render_cv_detail(id: i64) -> content::RawJson<String> {
    let gen_cv = get_render_cv_by_id(id);
    if let Some(v) = gen_cv {
        return box_rest_response(v);
    } else {
        return box_rest_response("no data");
    }
}

/// # 根据ID删除简历
///
/// 根据ID删除简历
#[openapi(tag = "简历")]
#[delete("/v1/cv/<id>")]
pub fn del_cv(id: i64, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let del_result = del_cv_by_id(id, &login_user_info);
    match del_result {
        Ok(_v) => {
            return box_rest_response(id);
        }
        Err(e) => {
            return box_error_rest_response("-1", "500".to_string(), e.to_string());
        }
    }
}

/// # 根据ID查询简历基础信息
///
/// 根据ID查询简历基础信息
#[openapi(tag = "简历")]
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
#[openapi(tag = "简历")]
#[post("/v1/cv", data = "<request>")]
pub fn edit_cv_summary(
    request: Json<EditMainRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_cv_main(&request, &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 更新简历排序信息
///
/// 更新简历排序信息
#[openapi(tag = "简历")]
#[put("/v1/cv-order", data = "<request>")]
pub fn edit_cv_sort(
    request: Json<EditMainSort>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_cv_main_sort(&request, &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 复制简历
///
/// 复制简历
#[openapi(tag = "简历")]
#[post("/v1/cv-copy", data = "<request>")]
pub fn copy_cv(
    request: Json<CopyMainCv>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = copy_cv_main(&request, &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 更新简历模版
///
/// 更新简历模版
#[openapi(tag = "简历")]
#[put("/v1/tpl", data = "<request>")]
pub fn update_cv_tpl(
    request: Json<UpdateMainCvTpl>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_cv_template(&request.cv_id, &request.tpl_id, &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 更新简历主色调
///
/// 更新简历主色调
#[openapi(tag = "简历")]
#[put("/v1/color", data = "<request>")]
pub fn update_cv_color(
    request: Json<UpdateMainCvColor>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_cv_main_color(&request, &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 更新简历主题
///
/// 更新简历主题
#[openapi(tag = "简历")]
#[put("/v1/theme", data = "<request>")]
pub fn update_cv_theme(
    request: Json<UpdateMainCvConfig>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_cv_main_config(&request, &login_user_info);
    return box_rest_response(gen_cv);
}

/// # 更新简历字体/主题/色调
///
/// 更新简历字体/主题/色调
#[openapi(tag = "简历")]
#[put("/v1/config", data = "<request>")]
pub fn update_cv_config(
    request: Json<UpdateMainCvConfig>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    let gen_cv = update_cv_main_config(&request, &login_user_info);
    return box_rest_response(gen_cv);
}
