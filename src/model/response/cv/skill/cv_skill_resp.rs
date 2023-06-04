use crate::model::diesel::cv::custom_cv_models::CvSkill;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Deserialize, Default, Clone, JsonSchema)]
pub struct CvSkillResp {
    pub id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub user_id: i64,
    pub name: String,
    pub memo: Option<String>
}

impl From<&CvSkill> for CvSkillResp {
    fn from(s: &CvSkill) -> Self {
        Self {
            id: s.id,
            created_time: s.created_time,
            updated_time: s.updated_time,
            cv_id: s.cv_id,
            user_id: s.user_id,
            name: s.name.clone(),
            memo: s.memo.clone(),
        }
    }
}
