use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvGen, CvMain, CvSection, CvSectionContent};
use crate::model::response::cv::cv_main_resp::CvMainResp;
use crate::model::response::cv::cv_section_resp::CvSectionResp;
use crate::model::response::cv::section_content_resp::SectionContentResp;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use rust_wheel::common::util::model_convert::map_entity;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn cv_main_list(login_user_info: &LoginUserInfo) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get user bill book");
    return user_bill_books;
}

pub fn get_cv_by_id(cv_id: i64, login_user_info: &LoginUserInfo) -> CvMainResp {
    use crate::model::diesel::cv::cv_schema::cv_main as cv_main_table;
    let mut query = cv_main_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(
        cv_main_table::user_id
            .eq(login_user_info.userId)
            .and(cv_main_table::id.eq(cv_id)),
    );
    let cv_result: Vec<CvMain> = query
        .load::<CvMain>(&mut get_connection())
        .expect("error get cv");
    let section_resp = get_section_by_cv(cv_id);
    let cv_resp = CvMainResp::from(&cv_result.get(0).unwrap(), section_resp);
    return cv_resp;
}

pub fn get_content_by_section(section_ids: Vec<i64>) -> Vec<SectionContentResp> {
    use crate::model::diesel::cv::cv_schema::cv_section_content as cv_section_content_table;
    let mut query = cv_section_content_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_section_content_table::section_id.eq_any(section_ids));
    let sections: Vec<CvSectionContent> = query
        .load::<CvSectionContent>(&mut get_connection())
        .expect("error get user bill book");
    let content_resp: Vec<SectionContentResp> = map_entity(sections);
    return content_resp;
}

pub fn get_section_by_cv(cv_id: i64) -> Vec<CvSectionResp> {
    use crate::model::diesel::cv::cv_schema::cv_section as cv_section_table;
    let mut query = cv_section_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_section_table::cv_id.eq(cv_id));
    let sections: Vec<CvSection> = query
        .load::<CvSection>(&mut get_connection())
        .expect("error get user bill book");
    let mut sec_resp: Vec<CvSectionResp> = map_entity(sections.clone());
    let section_ids: Vec<i64> = sections.iter().map(|item| item.id).collect();
    let content_resp = get_content_by_section(section_ids);
    for mut sec_item in sec_resp.iter_mut() {
        let item_id: i64 = sec_item.id;
        let contents:Vec<_> = content_resp.iter().filter(|item| item.section_id == item_id)
        .map(|section_content_resp| section_content_resp.to_owned())
        .collect();
        sec_item.section_content = contents;
    }
    return sec_resp;
}
