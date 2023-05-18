use crate::{common::health_controller, biz::cv::gen_controller};
use rocket::{Build, Rocket};
use rocket_okapi::{
    mount_endpoints_and_merged_docs,
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig},
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

pub fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build()
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
        "/gen" => gen_controller::get_routes_and_docs(&openapi_settings)
    };
    building_rocket
}