use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::cv::cv_schema::*;
use crate::model::request::cv::skills::skills_request::SkillsRequest;
use crate::model::response::cv::skill::cv_skill_resp::CvSkillResp;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_skills)]
pub struct CvSkillAdd {
    pub name: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub level: Option<String>,
    pub user_id: i64,
    pub memo: Option<String>,
}

impl CvSkillAdd {

    pub(crate) fn from_work_resp(request: &CvSkillResp, login_user_info: &LoginUserInfo) ->Self {
        Self {
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            cv_id: request.cv_id.clone(),
            name: request.name.clone(),
            level: Some("".to_string()),
            memo: request.memo.clone(),
        }
    }

    pub(crate) fn from_req(request: &SkillsRequest, login_user_info: &LoginUserInfo) ->Self {
        Self {
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            cv_id: request.cv_id.clone(),
            name: request.name.clone(),
            level: Some("".to_string()),
            memo: request.memo.clone(),
        }
    }
}