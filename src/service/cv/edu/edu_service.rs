use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvEdu, CvEduAdd};
use crate::model::request::cv::edu::edu_request::EduRequest;
use chrono::NaiveDate;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn add_edu(request: &Json<EduRequest>, login_user_info: &LoginUserInfo) -> Vec<CvEdu> {
    use crate::model::diesel::cv::cv_schema::cv_edu::dsl::*;
    let admission_dt =
        NaiveDate::parse_from_str(&request.admission.to_string(), "%Y-%m-%d").unwrap();
    let graduation_dt =
        NaiveDate::parse_from_str(&request.admission.to_string(), "%Y-%m-%d").unwrap();
    let cv_edu_model = CvEduAdd {
        edu_addr: request.edu_addr.to_string(),
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        cv_id: request.cv_id,
        user_id: login_user_info.userId,
        degree: Some(request.degree.to_string()),
        major: Some(request.graduation.to_string()),
        admission: Some(admission_dt),
        graduation: Some(graduation_dt),
    };
    let result = diesel::insert_into(cv_edu)
        .values(&cv_edu_model)
        .on_conflict(id)
        .do_update()
        .set((
            edu_addr.eq(&request.edu_addr),
            (updated_time.eq(get_current_millisecond())),
        ))
        .get_result::<CvEdu>(&mut get_connection());
    match result {
        Err(_) => {
            print!("error")
        }
        Ok(_) => {
            print!("ok")
        }
    }
    let cv_edu_info = get_edu_list(&request.cv_id, &login_user_info);
    return cv_edu_info;
}

pub fn get_edu_list(cv_id: &i64, login_user_info: &LoginUserInfo) -> Vec<CvEdu> {
    use crate::model::diesel::cv::cv_schema::cv_edu as cv_edu_table;
    let mut query = cv_edu_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_edu_table::user_id.eq(login_user_info.userId));
    query = query.filter(cv_edu_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvEdu>(&mut get_connection())
        .expect("error get edu list");
    return cvs;
}

pub fn del_edu_item(item_id: &i64, login_user_info: &LoginUserInfo) -> bool {
    use crate::model::diesel::cv::cv_schema::cv_edu::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_edu::id
        .eq(item_id)
        .and(crate::model::diesel::cv::cv_schema::cv_edu::user_id.eq(login_user_info.userId));
    let delete_result = diesel::delete(cv_edu.filter(predicate)).execute(&mut get_connection());
    match delete_result {
        Ok(_v) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

pub fn del_edu_items(del_cv_id: &i64, login_user_info: &LoginUserInfo) -> bool {
    use crate::model::diesel::cv::cv_schema::cv_edu::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_edu::cv_id
        .eq(del_cv_id)
        .and(crate::model::diesel::cv::cv_schema::cv_edu::user_id.eq(login_user_info.userId));
    let delete_result = diesel::delete(cv_edu.filter(predicate)).execute(&mut get_connection());
    match delete_result {
        Ok(_v) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}