use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvGen, CvGenAdd};
use crate::model::request::cv::gen_request::GenRequest;
use crate::model::request::cv::render_result_request::RenderResultRequest;
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods, BoolExpressionMethods};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn cv_gen_list(filter_name: Option<String>, login_user_info: &LoginUserInfo) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!(
            "{}{}{}",
            "%",
            some_filter_name.as_str(),
            "%"
        )));
    }
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get user gen record");
    return user_bill_books;
}

pub fn cv_gen_page(filter_name: Option<String>, login_user_info: &LoginUserInfo) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!(
            "{}{}{}",
            "%",
            some_filter_name.as_str(),
            "%"
        )));
    }
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get user gen record");
    return user_bill_books;
}

pub fn create_gen_task(
    request: &Json<GenRequest>,
    login_user_info: &LoginUserInfo,
) -> Result<CvGen, diesel::result::Error> {
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let cv_gen_add = CvGenAdd {
        cv_name: request.cv_name.to_owned(),
        remark: "".to_owned(),
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        user_id: login_user_info.userId,
        gen_status: 0,
        gen_time: Some(0),
        path: Some("".to_owned()),
        template_id: request.template_id,
        cv_id: request.cv_id,
    };
    let result = diesel::insert_into(cv_gen)
        .values(&cv_gen_add)
        .on_conflict(id)
        .do_update()
        .set(((updated_time.eq(get_current_millisecond())),))
        .get_result::<CvGen>(&mut get_connection());
    return result;
}

pub fn cv_gen_list_render(filter_name: Option<String>) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!(
            "{}{}{}",
            "%",
            some_filter_name.as_str(),
            "%"
        )));
    }
    query = query.filter(cv_gen_table::gen_status.eq(0));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get render gen record");
    return user_bill_books;
}

pub fn update_gen_result(request: Json<RenderResultRequest>) {
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_gen::id.eq(request.id);
    diesel::update(cv_gen.filter(predicate))
        .set((
            gen_status.eq(&request.gen_status),
            (path.eq(request.path.to_string())),
            (updated_time.eq(get_current_millisecond())),
        ))
        .get_result::<CvGen>(&mut get_connection())
        .expect("unable to update ren result");
}

pub fn del_gen_impl(gen_id: &i64, login_user_info: &LoginUserInfo)  -> bool{
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_gen::id.eq(gen_id)
    .and(crate::model::diesel::cv::cv_schema::cv_gen::user_id.eq(login_user_info.userId));
    let delete_result =
        diesel::delete(cv_gen.filter(predicate)).execute(&mut get_connection());
    match delete_result {
        Ok(_v) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}