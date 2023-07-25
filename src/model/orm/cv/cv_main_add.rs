use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::cv::cv_schema::*;
use crate::model::request::cv::main::edit_main_request::EditMainRequest;
use crate::model::response::cv::cv_main_resp::CvMainResp;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_main)]
pub struct CvMainAdd {
    pub cv_name: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_id: i64,
    pub cv_status: i32,
    pub template_id: i64,
    pub employee_name: Option<String>,
    pub birthday: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub job: Option<String>,
    pub workplace: Option<String>,
    pub item_order: String,
    pub remark: Option<String>,
    pub stackoverflow: Option<String>,
    pub github: Option<String>,
    pub blog: Option<String>,
}

impl CvMainAdd {
    pub(crate) fn from(request: &EditMainRequest, login_user_info: &LoginUserInfo) ->Self {
        Self {
            cv_name: request.cv_name.to_string(),
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            cv_status: 0,
            template_id: 1,
            employee_name: Some(request.employee_name.to_string()),
            birthday: Some(request.birthday.to_string()),
            phone: Some(request.phone.to_string()),
            email: Some(request.email.to_string()),
            job: Some(request.job.to_string()),
            workplace: Some(request.workplace.to_string()),
            item_order: "1,2,3,4,5,6".to_string(),
            remark: request.remark.clone(),
            stackoverflow: request.stackoverflow.clone(),
            github: request.github.clone(),
            blog: request.blog.clone(),  
        }
    }

    pub(crate) fn from_resp(request: &CvMainResp, login_user_info: &LoginUserInfo) ->Self {
        Self {
            cv_name: request.cv_name.to_string(),
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            user_id: login_user_info.userId,
            cv_status: 0,
            template_id: 1,
            employee_name: request.employee_name.clone(),
            birthday: request.birthday.clone(),
            phone: request.phone.clone(),
            email: request.email.clone(),
            job: request.job.clone(),
            workplace: request.workplace.clone(),
            item_order: "1,2,3,4,5".to_string(),
            remark: request.remark.clone(),
            stackoverflow: request.stackoverflow.clone(),
            github: request.github.clone(),
            blog: request.blog.clone(),   
        }
    }
}
