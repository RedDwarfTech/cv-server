use chrono::NaiveDate;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::cv::cv_schema::*;
use crate::model::request::cv::edu::edu_request::EduRequest;
use crate::model::response::cv::edu::cv_edu_resp::CvEduResp;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_edu)]
pub struct CvEduAdd {
    pub edu_addr: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub user_id: i64,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub city: Option<String>,
    pub admission: Option<NaiveDate>,
    pub graduation: Option<NaiveDate>,
}

impl CvEduAdd {

    pub(crate) fn from_request(request: &EduRequest, login_user_info: &LoginUserInfo) ->Self {
        let admission_dt =
        NaiveDate::parse_from_str(&request.admission.to_string(), "%Y-%m-%d").unwrap();
    let graduation_dt =
        NaiveDate::parse_from_str(&request.graduation.to_string(), "%Y-%m-%d").unwrap();
        Self {
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            edu_addr: request.edu_addr.clone(),
            cv_id: request.cv_id.clone(),
            degree: Some(request.degree.clone()),
            major: Some(request.major.clone()),
            city: request.city.clone(),
            admission: Some(admission_dt),
            graduation: Some(graduation_dt), 
        }
    }

    pub(crate) fn from_edu_resp(request: &CvEduResp, login_user_info: &LoginUserInfo) ->Self {
        let admission_dt =
        NaiveDate::parse_from_str(&request.admission.as_deref().unwrap_or_default(), "%Y-%m-%d").unwrap();
    let graduation_dt =
        NaiveDate::parse_from_str(&request.graduation.as_deref().unwrap_or_default(), "%Y-%m-%d").unwrap();
        Self {
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            edu_addr: request.edu_addr.clone(),
            cv_id: request.cv_id.clone(),
            degree: request.degree.clone(),
            major: request.major.clone(),
            city: request.city.clone(),
            admission: Some(admission_dt),
            graduation: Some(graduation_dt), 
        }
    }
}