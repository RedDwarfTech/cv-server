use okapi::openapi3::OpenApi;
use rocket::{response::content, get};
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rust_wheel::{
    common::util::model_convert::box_rest_response
};
use crate::service::template::template_service::{get_tempalte_by_id, get_tempalte_list};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get,get_list]
}

/// # 查询简历模版
///
/// 简历模版
#[openapi(tag = "简历模版")]
#[get("/v1/<id>")]
pub fn get(id: i64) -> content::RawJson<String> {
    let gen_cv = get_tempalte_by_id(id);
    return box_rest_response(gen_cv);
}

/// # 查询简历模版列表
///
/// 查询简历模版列表
#[openapi(tag = "简历模版")]
#[get("/v1/list?<ids>")]
pub fn get_list(ids: Option<String>) -> content::RawJson<String> {
    if let Some(ids) = ids {
        let id_arr = ids.split(",").collect::<Vec<&str>>();
        let numbers: Vec<i64> = id_arr
        .iter()
        .map(|&part| part.parse::<i64>().unwrap())
        .collect();
        let gen_cv = get_tempalte_list(Some(numbers));
        return box_rest_response(gen_cv);
    }else{
        let gen_cv = get_tempalte_list(None);
        return box_rest_response(gen_cv);
    }
}