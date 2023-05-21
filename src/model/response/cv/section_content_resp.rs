use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::model::diesel::cv::custom_cv_models::CvSectionContent;

#[derive( Serialize, Queryable, Deserialize,Default, Clone, JsonSchema)]
pub struct SectionContentResp {
    pub id: i64,
    pub section_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub item_key: Option<String>,
    pub item_value: Option<String>,
}

impl From<&CvSectionContent> for SectionContentResp {
    fn from(c: &CvSectionContent) -> Self {
        Self {
            id: c.id,
            section_id: c.section_id,
            created_time: c.created_time,
            updated_time: c.updated_time,
            item_key: c.item_key.to_owned(),
            item_value: c.item_value.to_owned(),
        }
    }
}