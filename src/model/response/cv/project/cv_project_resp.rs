use serde::{Serialize, Deserialize};
use crate::model::diesel::cv::custom_cv_models::{ CvProjectExp};
use schemars::JsonSchema;

#[derive(Serialize, Queryable, Deserialize, Default, Clone, JsonSchema)]
pub struct CvProjectResp {
    pub id: i64,
    pub name: String,
    pub company: Option<String>,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub job: Option<String>,
    pub work_start: Option<String>,
    pub work_end: Option<String>,
    pub user_id: i64,
    pub duty: Option<String>,
}

impl From<&CvProjectExp> for CvProjectResp {
    fn from(s: &CvProjectExp) -> Self {
        Self {
            id: s.id,
            created_time: s.created_time,
            updated_time: s.updated_time,
            cv_id: s.cv_id,
            user_id: s.user_id,
            company: s.company.clone(),
            name: s.name.clone(),
            job: s.job.to_owned(),
            work_start: Some(s.work_start.unwrap().to_string()),
            work_end: Some(s.work_end.unwrap().to_string()),
            duty: s.duty.clone(),
        }
    }
}