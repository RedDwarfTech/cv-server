use chrono::NaiveDate;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::cv::cv_schema::*;
use crate::model::request::cv::work::work_request::WorkRequest;
use crate::model::response::cv::work::cv_work_resp::CvWorkResp;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_work_exp)]
pub struct CvWorkExpAdd {
    pub company: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub job: Option<String>,
    pub city: Option<String>,
    pub work_start: Option<NaiveDate>,
    pub work_end: Option<NaiveDate>,
    pub user_id: i64,
    pub duty: Option<String>,
}

impl CvWorkExpAdd {

    pub(crate) fn from_request(request: &WorkRequest, login_user_info: &LoginUserInfo) ->Self {
        let admission_dt =
        NaiveDate::parse_from_str(&request.work_start.to_string(), "%Y-%m-%d").unwrap();
    let graduation_dt =
        NaiveDate::parse_from_str(&request.work_end.to_string(), "%Y-%m-%d").unwrap();
        Self {
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            cv_id: request.cv_id.clone(),
            city: Some(request.city.clone()),
            work_start: Some(admission_dt),
            work_end: Some(graduation_dt),
            company: request.company.clone(),
            job: Some(request.job.clone()),
            duty: request.duty.clone(), 
        }
    }

    pub(crate) fn from_work_resp(request: &CvWorkResp, login_user_info: &LoginUserInfo) ->Self {
        let admission_dt =
        NaiveDate::parse_from_str(&request.work_start.as_deref().unwrap_or_default(), "%Y-%m-%d").unwrap();
    let graduation_dt =
        NaiveDate::parse_from_str(&request.work_end.as_deref().unwrap_or_default(), "%Y-%m-%d").unwrap();
        Self {
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            cv_id: request.cv_id.clone(),
            city: request.city.clone(),
            work_start: Some(admission_dt),
            work_end: Some(graduation_dt),
            company: request.company.clone(),
            job: request.job.clone(),
            duty: request.duty.clone(), 
        }
    }
}