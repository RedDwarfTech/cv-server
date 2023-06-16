use crate::model::diesel::cv::custom_cv_models::{CvGen};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Deserialize, Default, Clone, JsonSchema)]
pub struct CvGenResp {
    pub id: i64,
    pub cv_name: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_id: i64,
    pub gen_status: i32,
    pub gen_time: Option<i64>,
    pub path: Option<String>,
    pub template_id: i64,
    pub cv_id: i64,
    pub template_name: Option<String>,
    pub preview_url: Option<String>,
}

impl From<&CvGen> for CvGenResp {
    fn from(cv_gen: &CvGen) -> Self {
        Self {
            id: cv_gen.id,
            cv_name: cv_gen.cv_name.to_string(),
            created_time: cv_gen.created_time,
            updated_time: cv_gen.updated_time,
            user_id: cv_gen.user_id,
            template_id: cv_gen.template_id,
            remark: cv_gen.remark.to_string(),
            gen_status: cv_gen.gen_status,
            gen_time: cv_gen.gen_time,
            path: cv_gen.path.to_owned(),
            cv_id: cv_gen.cv_id,
            template_name: None,
            preview_url: None,
        }
    }
}
