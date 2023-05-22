// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDate;
use diesel::sql_types::Date;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::cv::cv_schema::*;
use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
// https://stackoverflow.com/questions/76282080/is-it-possible-to-make-rust-recognized-the-new-rust-diesel-model
#[diesel(table_name = cv_gen)]
pub struct CvGen {
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
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_template)]
pub struct CvTemplate {
    pub id: i64,
    pub name: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub template_status: i32,
    pub template_id: i64,
    pub preview_url: Option<String>,
    pub template_code: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_main)]
pub struct CvMain {
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
    pub job: Option<String>,
    pub workplace: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_section)]
pub struct CvSection {
    pub id: i64,
    pub section_abbr: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: Option<i64>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_section_content)]
pub struct CvSectionContent {
    pub id: i64,
    pub section_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub item_key: Option<String>,
    pub item_value: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_section_type)]
pub struct CvSectionType {
    pub id: i64,
    pub item_name: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub item_abbr: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_edu)]
pub struct CvEdu {
    pub id: i64,
    pub edu_addr: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub admission: Option<NaiveDate>,
    pub graduation: Option<NaiveDate>,
    pub user_id: i64,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = cv_edu)]
pub struct CvEduAdd {
    pub edu_addr: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cv_id: i64,
    pub user_id: i64,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub admission: Option<NaiveDate>,
    pub graduation: Option<NaiveDate>,
}