use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvMain, CvSection, CvSectionContent};
use crate::model::orm::cv::cv_main_add::CvMainAdd;
use crate::model::request::cv::main::edit_main_request::EditMainRequest;
use crate::model::response::cv::cv_main_resp::CvMainResp;
use crate::model::response::cv::cv_section_resp::CvSectionResp;
use crate::model::response::cv::section_content_resp::SectionContentResp;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::map_entity;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn cv_main_list(login_user_info: &LoginUserInfo) -> Vec<CvMain> {
    use crate::model::diesel::cv::cv_schema::cv_main as cv_main_table;
    let mut query = cv_main_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_main_table::user_id.eq(login_user_info.userId));
    let cvs = query
        .load::<CvMain>(&mut get_connection())
        .expect("error get cv main");
    return cvs;
}

pub fn get_cv_summary(cv_id: i64, login_user_info: &LoginUserInfo) -> Option<CvMain>{
    use crate::model::diesel::cv::cv_schema::cv_main as cv_main_table;
    let mut query = cv_main_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(
        cv_main_table::user_id
            .eq(login_user_info.userId)
            .and(cv_main_table::id.eq(cv_id)),
    );
    let cv_result: Vec<CvMain> = query
        .load::<CvMain>(&mut get_connection())
        .expect("error get cv summary");
    if cv_result.len() > 0 {
        return Some(cv_result.get(0).unwrap().clone());
    }else{
        return None;
    }
}

pub fn get_cv_by_id(cv_id: i64, login_user_info: &LoginUserInfo) -> Option<CvMainResp> {
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
    if cv_result.is_empty() {
        return None;
    }
    let section_resp = get_section_by_cv(cv_id);
    let cv_resp = CvMainResp::from(&cv_result.get(0).unwrap(), section_resp);
    return Some(cv_resp);
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
        let contents: Vec<_> = content_resp
            .iter()
            .filter(|item| item.section_id == item_id)
            .map(|section_content_resp| section_content_resp.to_owned())
            .collect();
        sec_item.section_content = contents;
    }
    return sec_resp;
}

pub fn update_cv_main(request: &Json<EditMainRequest>, login_user_info: &LoginUserInfo) -> Option<CvMain>{
    use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
    if request.id.is_some() {
        let predicate = crate::model::diesel::cv::cv_schema::cv_main::id.eq(request.id.unwrap()).and(
            crate::model::diesel::cv::cv_schema::cv_main::user_id.eq(login_user_info.userId)
        );
        let update_result = diesel::update(cv_main.filter(predicate))
        .set((
            employee_name.eq(&request.employee_name),
            job.eq(&request.job)
        ))
        .get_result::<CvMain>(&mut get_connection())
        .expect("unable to update ren result");
        return Some(update_result);
    }else{
        let cv_summary = CvMainAdd::from(request,login_user_info);
        let result = diesel::insert_into(cv_main)
        .values(&cv_summary)
        .on_conflict(id)
        .do_update()
        .set((
            employee_name.eq(&request.employee_name),
            (updated_time.eq(get_current_millisecond())),
            job.eq(&request.job)
        ))
        .get_result::<CvMain>(&mut get_connection());
    match result {
        Err(err) => {
            println!("{}", err);
            return None;
        }
        Ok(main) => {
            return Some(main);
        }
    }
    }
}
