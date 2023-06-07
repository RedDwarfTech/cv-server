use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvWorkExp, CvWorkExpAdd};
use crate::model::request::cv::work::work_request::WorkRequest;
use chrono::NaiveDate;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn add_work(
    request: &Json<WorkRequest>,
    login_user_info: &LoginUserInfo,
) -> Result<CvWorkExp, diesel::result::Error> {
    use crate::model::diesel::cv::cv_schema::cv_work_exp::dsl::*;
    let admission_dt =
        NaiveDate::parse_from_str(&request.work_start.to_string(), "%Y-%m-%d").unwrap();
    let graduation_dt =
        NaiveDate::parse_from_str(&request.work_end.to_string(), "%Y-%m-%d").unwrap();
    let cv_edu_model = CvWorkExpAdd {
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        cv_id: request.cv_id,
        user_id: login_user_info.userId,
        work_start: Some(admission_dt),
        work_end: Some(graduation_dt),
        company: request.company.clone(),
        job: Some(request.job.clone()),
        city: Some(request.city.to_string()),
        duty: request.duty.clone(),
    };

    if request.id.is_some() {
        let predicate = crate::model::diesel::cv::cv_schema::cv_work_exp::id
            .eq(request.id.unwrap())
            .and(
                crate::model::diesel::cv::cv_schema::cv_work_exp::user_id
                    .eq(login_user_info.userId),
            );
        let update_result = diesel::update(cv_work_exp.filter(predicate))
            .set((
                updated_time.eq(get_current_millisecond()),
                duty.eq(request.duty.clone()),
                company.eq(request.company.clone()),
                job.eq(request.job.clone()),
                city.eq(request.city.clone()),
                work_start.eq(admission_dt),
                work_end.eq(graduation_dt),
            ))
            .get_result::<CvWorkExp>(&mut get_connection());
        return update_result;
    } else {
        let result = diesel::insert_into(cv_work_exp)
            .values(&cv_edu_model)
            .on_conflict(id)
            .do_update()
            .set((
                updated_time.eq(get_current_millisecond()),
                duty.eq(request.duty.clone()),
                company.eq(request.company.clone()),
                job.eq(request.job.clone()),
                city.eq(request.city.clone()),
                work_start.eq(admission_dt),
                work_end.eq(graduation_dt),
            ))
            .get_result::<CvWorkExp>(&mut get_connection());
        return result;
    }
}

pub fn get_ui_work_list(cv_id: &i64, login_user_info: &LoginUserInfo) -> Vec<CvWorkExp> {
    use crate::model::diesel::cv::cv_schema::cv_work_exp as cv_work_table;
    let mut query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_work_table::user_id.eq(login_user_info.userId));
    query = query.filter(cv_work_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvWorkExp>(&mut get_connection())
        .expect("error get work list");
    return cvs;
}

pub fn get_work_list(cv_id: &i64) -> Vec<CvWorkExp> {
    use crate::model::diesel::cv::cv_schema::cv_work_exp as cv_work_table;
    let mut query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_work_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvWorkExp>(&mut get_connection())
        .expect("error get work list");
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

pub fn del_work_items(del_cv_id: &i64, login_user_info: &LoginUserInfo) -> bool {
    use crate::model::diesel::cv::cv_schema::cv_work_exp::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_work_exp::cv_id
        .eq(del_cv_id)
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
