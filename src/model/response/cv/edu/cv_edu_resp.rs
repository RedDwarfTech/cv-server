use serde::{Serialize, Deserialize};
use crate::model::diesel::cv::custom_cv_models::{CvEdu};
use schemars::JsonSchema;

#[derive(Serialize, Queryable, Deserialize, Default, Clone, JsonSchema)]
pub struct CvEduResp {
    pub id: i64,
    pub edu_addr: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub user_id: i64,
    pub admission: Option<String>,
    pub graduation: Option<String>,
}

impl From<&CvEdu> for CvEduResp {
    fn from(s: &CvEdu) -> Self {
        Self {
            id: s.id,
            created_time: s.created_time,
            updated_time: s.updated_time,
            cv_id: s.cv_id,
            edu_addr: s.edu_addr.to_string(),
            degree: s.degree.clone(),
            major: s.major.clone(),
            user_id: s.user_id,
            admission: Some(s.admission.unwrap().to_string()),
            graduation: Some(s.graduation.unwrap().to_string()),
        }
    }
}