use chrono::NaiveDate;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::cv::cv_schema::*;
use crate::model::response::cv::project::cv_project_resp::CvProjectResp;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_project_exp)]
pub struct CvProjectExpAdd {
    pub name: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub job: Option<String>,
    pub company: Option<String>,
    pub work_start: Option<NaiveDate>,
    pub work_end: Option<NaiveDate>,
    pub user_id: i64,
    pub duty: Option<String>,
    pub city: Option<String>,
}

impl CvProjectExpAdd {

    pub(crate) fn from_work_resp(request: &CvProjectResp, login_user_info: &LoginUserInfo) ->Self {
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
            name: request.name.clone(), 
        }
    }
}