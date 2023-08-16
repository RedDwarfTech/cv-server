use crate::{
    biz::{
        cv::{cv_main_controller, gen_controller, edu::edu_controller,work::work_exp_controller, skills::skills_controller, project::project_exp_controller, lang::lang_controller},
        template::cv_template_controller,
    },
    common::health_controller,
};
use log::error;
use rocket::{Build, Rocket, catchers, catch};
use rocket_okapi::{
    mount_endpoints_and_merged_docs,
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig},
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

#[catch(500)]
fn internal_error(_req: &rocket::Request) -> String{
    error!("internal error");
    return "internal error".to_string();
}

pub fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build().register("/", catchers![internal_error])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../cv/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("RedDwarf CV api | RedDwarfapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );
    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/cv".to_owned(), openapi_settings,
        "/actuator" => health_controller::get_routes_and_docs(&openapi_settings),
        "/gen" => gen_controller::get_routes_and_docs(&openapi_settings),
        "/cv" => cv_main_controller::get_routes_and_docs(&openapi_settings),
        "/tpl" => cv_template_controller::get_routes_and_docs(&openapi_settings),
        "/cv/edu" => edu_controller::get_routes_and_docs(&openapi_settings),
        "/cv/work" => work_exp_controller::get_routes_and_docs(&openapi_settings),
        "/cv/skills" => skills_controller::get_routes_and_docs(&openapi_settings),
        "/cv/project" => project_exp_controller::get_routes_and_docs(&openapi_settings),
        "/cv/lang" => lang_controller::get_routes_and_docs(&openapi_settings),
    };
    building_rocket
}
