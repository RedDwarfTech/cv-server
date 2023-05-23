use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvWorkExp, CvWorkExpAdd};
use crate::model::request::cv::edu::edu_request::EduRequest;
use chrono::NaiveDate;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn add_work(request: &Json<EduRequest>, login_user_info: &LoginUserInfo) -> Vec<CvWorkExp> {
    use crate::model::diesel::cv::cv_schema::cv_work_exp::dsl::*;
    let admission_dt =
        NaiveDate::parse_from_str(&request.admission.to_string(), "%Y-%m-%d").unwrap();
    let graduation_dt =
        NaiveDate::parse_from_str(&request.admission.to_string(), "%Y-%m-%d").unwrap();
    let cv_edu_model = CvWorkExpAdd {
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        cv_id: request.cv_id,
        user_id: login_user_info.userId,
        admission: Some(admission_dt),
        graduation: Some(graduation_dt),
        company: "reddwarf".to_owned(),
        job: Some("frontend".to_owned()),
        city: Some("todo!()".to_owned()),
    };
    let result = diesel::insert_into(cv_work_exp)
        .values(&cv_edu_model)
        .on_conflict(id)
        .do_update()
        .set(((updated_time.eq(get_current_millisecond())),))
        .get_result::<CvWorkExp>(&mut get_connection());
    match result {
        Err(_) => {
            print!("error")
        }
        Ok(_) => {
            print!("ok")
        }
    }
    let cv_edu_info: Vec<CvWorkExp> = get_work_list(&request.cv_id, &login_user_info);
    return cv_edu_info;
}

pub fn get_work_list(cv_id: &i64, login_user_info: &LoginUserInfo) -> Vec<CvWorkExp> {
    use crate::model::diesel::cv::cv_schema::cv_work_exp as cv_work_table;
    let mut query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_work_table::user_id.eq(login_user_info.userId));
    query = query.filter(cv_work_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvWorkExp>(&mut get_connection())
        .expect("error get edu list");
    return cvs;
}

pub fn del_work_item(item_id: &i64, login_user_info: &LoginUserInfo) -> bool {
    use crate::model::diesel::cv::cv_schema::cv_work_exp::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_work_exp::id
        .eq(item_id)
        .and(crate::model::diesel::cv::cv_schema::cv_work_exp::user_id.eq(login_user_info.userId));
    let delete_result =
        diesel::delete(cv_work_exp.filter(predicate)).execute(&mut get_connection());
    match delete_result {
        Ok(_v) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}
