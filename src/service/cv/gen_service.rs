use diesel::{QueryDsl, ExpressionMethods, TextExpressionMethods, BoolExpressionMethods};
use rocket::serde::json::Json;
use rust_wheel::{model::user::login_user_info::LoginUserInfo};
use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::CvGen;
use crate::model::request::cv::render_result_request::RenderResultRequest;

pub fn cv_gen_list(filter_name: Option<String>,login_user_info: &LoginUserInfo) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!("{}{}{}","%",some_filter_name.as_str(),"%")));
    }
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get user bill book");
    return user_bill_books;
}

pub fn update_gen_result(request: Json<RenderResultRequest>,login_user_info: &LoginUserInfo) {
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_gen::id.eq(request.id).and(
        crate::model::diesel::cv::cv_schema::cv_gen::user_id.eq(login_user_info.userId)
    );
    diesel::update(cv_gen.filter(predicate))
    .set(gen_status.eq(&request.gen_status))
    .get_result::<CvGen>(&mut get_connection())
    .expect("unable to update ren result");
}
