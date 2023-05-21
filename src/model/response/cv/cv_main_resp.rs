use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::model::diesel::cv::custom_cv_models::CvMain;

use super::{cv_section_resp::CvSectionResp};

#[derive( Serialize, Queryable, Deserialize,Default, Clone, JsonSchema)]
pub struct CvMainResp {
    pub id: i64,
    pub cv_name: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_id: i64,
    pub cv_status: i32,
    pub template_id: i64,
    pub employee_name: Option<String>,
    pub birthday: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cv_section: Vec<CvSectionResp>
}

impl CvMainResp {
    pub(crate) fn from(cv_main: &CvMain, sections: Vec<CvSectionResp>) ->Self {
        Self {
            id: cv_main.id,
            cv_name: cv_main.cv_name.to_string(),
            remark: cv_main.remark.to_string(),
            created_time: cv_main.created_time,
            updated_time: cv_main.updated_time,
            user_id: cv_main.user_id,
            cv_status: cv_main.cv_status,
            template_id: cv_main.template_id,
            employee_name: cv_main.employee_name.to_owned(),
            birthday: cv_main.birthday.to_owned(),
            phone: cv_main.phone.to_owned(),
            email: cv_main.email.to_owned(),
            cv_section: sections,
        }
    }
}
