use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::model::diesel::cv::custom_cv_models::CvSection;
use super::section_content_resp::SectionContentResp;

#[derive( Serialize, Queryable, Deserialize,Default, Clone, JsonSchema)]
pub struct CvSectionResp {
    pub id: i64,
    pub section_abbr: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: Option<i64>,
    pub section_content: Vec<SectionContentResp>
}

impl From<&CvSection> for CvSectionResp {
    fn from(s: &CvSection) -> Self {
        Self {
            id: s.id,
            section_abbr: s.section_abbr.to_string(),
            remark: s.remark.to_string(),
            created_time: s.created_time,
            updated_time: s.updated_time,
            cv_id: s.cv_id,
            section_content: [].to_vec(),
        }
    }
}