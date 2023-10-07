use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{
    CvEdu, CvMain, CvProjectExp, CvSection, CvSectionContent, CvSkill, CvWorkExp, CvTemplate, CvLang,
};
use crate::model::orm::cv::cv_main_add::CvMainAdd;
use crate::model::orm::cv::edu::cv_edu_add::CvEduAdd;
use crate::model::orm::cv::lang::cv_lang_add::CvLangAdd;
use crate::model::orm::cv::project::cv_project_add::CvProjectExpAdd;
use crate::model::orm::cv::skill::cv_skill_add::CvSkillAdd;
use crate::model::orm::cv::work::cv_work_add::CvWorkExpAdd;
use crate::model::request::cv::main::copy_main_cv::CopyMainCv;
use crate::model::request::cv::main::edit_main_request::EditMainRequest;
use crate::model::request::cv::main::edit_main_sort::EditMainSort;
use crate::model::request::cv::main::update_main_cv_color::UpdateMainCvColor;
use crate::model::request::cv::main::update_main_cv_config::UpdateMainCvConfig;
use crate::model::response::cv::cv_main_resp::CvMainResp;
use crate::model::response::cv::cv_section_resp::CvSectionResp;
use crate::model::response::cv::edu::cv_edu_resp::CvEduResp;
use crate::model::response::cv::lang::cv_lang_resp::CvLangResp;
use crate::model::response::cv::project::cv_project_resp::CvProjectResp;
use crate::model::response::cv::section_content_resp::SectionContentResp;
use crate::model::response::cv::skill::cv_skill_resp::CvSkillResp;
use crate::model::response::cv::work::cv_work_resp::CvWorkResp;
use crate::service::cv::edu::edu_service::{del_edu_items, get_edu_list};
use crate::service::cv::lang::lang_service::del_langs_items;
use crate::service::cv::project::project_exp_service::del_project_items;
use crate::service::cv::skills::skills_exp_service::del_skills_items;
use crate::service::cv::work::work_exp_service::{del_work_items, get_work_list};
use crate::service::template::template_service::get_tempalte_by_id;
use diesel::query_dsl::methods::BoxedDsl;
use diesel::result::Error;
use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl};
use log::error;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::map_entity;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use super::lang::lang_service::get_lang_list;
use super::project::project_exp_service::get_project_list;
use super::skills::skills_exp_service::get_skill_list;

pub fn cv_main_list(login_user_info: &LoginUserInfo) -> Vec<CvMain> {
    use crate::model::diesel::cv::cv_schema::cv_main as cv_main_table;
    let mut query = cv_main_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_main_table::user_id.eq(login_user_info.userId));
    let cvs = query
        .load::<CvMain>(&mut get_connection())
        .expect("error get cv main");
    return cvs;
}

pub fn get_cv_summary(cv_id: i64, login_user_info: &LoginUserInfo) -> Option<CvMain> {
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
    } else {
        return None;
    }
}

pub fn del_cv_by_id(cv_id: i64, login_user_info: &LoginUserInfo) -> Result<&str, Error> {
    use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
    let mut connection = get_connection();
    let result = connection.transaction(|connection| {
        use crate::model::diesel::cv::cv_schema::cv_main as cv_main_table;
        let predicate = cv_main_table::user_id
            .eq(login_user_info.userId)
            .and(cv_main_table::id.eq(cv_id));
        let delete_result = diesel::delete(cv_main.filter(predicate)).execute(connection);
        match delete_result {
            Ok(_v) => {
                del_edu_items(&cv_id, login_user_info);
                del_work_items(&cv_id, login_user_info);
                del_project_items(&cv_id, login_user_info);
                del_skills_items(&cv_id, login_user_info);
                del_langs_items(&cv_id, login_user_info);
                Ok("")
            }
            Err(e) => diesel::result::QueryResult::Err(e),
        }
    });
    return result;
}

pub fn get_cv_by_id(cv_id: i64, login_user_info: &LoginUserInfo) -> Option<CvMainResp> {
    use crate::model::diesel::cv::cv_schema::cv_main as cv_main_table;
    let mut query = cv_main_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(
        cv_main_table::user_id
            .eq(login_user_info.userId)
            .and(cv_main_table::id.eq(cv_id)),
    );
    return get_cv_info(cv_id, query);
}

pub fn get_render_cv_by_id(cv_id: i64) -> Option<CvMainResp> {
    use crate::model::diesel::cv::cv_schema::cv_main as cv_main_table;
    let mut query: <cv_main_table::table as BoxedDsl<diesel::pg::Pg>>::Output =
        cv_main_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_main_table::id.eq(cv_id));
    return get_cv_info(cv_id, query);
}

pub fn get_cv_info(
    cv_id: i64,
    query: <crate::model::diesel::cv::cv_schema::cv_main::table as BoxedDsl<diesel::pg::Pg>>::Output,
) -> Option<CvMainResp> {
    let cv_result: Vec<CvMain> = query
        .load::<CvMain>(&mut get_connection())
        .expect("error get cv");
    if cv_result.is_empty() {
        return None;
    }
    // edu
    let edues = get_edu_list(&cv_id);
    // work
    let works_list = get_work_list(&cv_id);
    // skill
    let skills: Vec<crate::model::diesel::cv::custom_cv_models::CvSkill> = get_skill_list(&cv_id);
    // project
    let projects: Vec<CvProjectExp> = get_project_list(&cv_id);
    // lang
    let langs: Vec<CvLang> = get_lang_list(&cv_id);
    let section_resp = get_section_by_cv(cv_id);
    let edu_resp: Vec<CvEduResp> = map_entity(edues);
    let works_resp: Vec<CvWorkResp> = map_entity(works_list);
    let skill_resp: Vec<CvSkillResp> = map_entity(skills);
    let projects_resp: Vec<CvProjectResp> = map_entity(projects);
    let lang_resp: Vec<CvLangResp> = map_entity(langs);
    let cv_resp = CvMainResp::from(
        &cv_result.get(0).unwrap(),
        section_resp,
        edu_resp,
        works_resp,
        skill_resp,
        projects_resp,
        lang_resp,
    );
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
    for sec_item in sec_resp.iter_mut() {
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

pub fn update_cv_main(
    request: &Json<EditMainRequest>,
    login_user_info: &LoginUserInfo,
) -> Option<CvMain> {
    use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
    if request.id.is_some() {
        let predicate = crate::model::diesel::cv::cv_schema::cv_main::id
            .eq(request.id.unwrap())
            .and(crate::model::diesel::cv::cv_schema::cv_main::user_id.eq(login_user_info.userId));
        let update_result = diesel::update(cv_main.filter(predicate))
            .set((
                employee_name.eq(&request.employee_name),
                job.eq(&request.job),
                workplace.eq(&request.workplace),
                phone.eq(&request.phone),
                email.eq(&request.email),
                birthday.eq(&request.birthday),
                cv_name.eq(&request.cv_name),
                stackoverflow.eq(&request.stackoverflow),
                github.eq(&request.github),
                blog.eq(&request.blog),
                remark.eq(&request.remark),
                main_color.eq("blue"),
                theme.eq("classic"),
                font_size.eq("10pt")
            ))
            .get_result::<CvMain>(&mut get_connection())
            .expect("unable to update cv main");
        return Some(update_result);
    } else {
        let cv_summary = CvMainAdd::from(request, login_user_info);
        let result = diesel::insert_into(cv_main)
            .values(&cv_summary)
            .on_conflict(id)
            .do_update()
            .set((
                employee_name.eq(&request.employee_name),
                (updated_time.eq(get_current_millisecond())),
                job.eq(&request.job),
            ))
            .get_result::<CvMain>(&mut get_connection());
        match result {
            Err(err) => {
                error!("{}", err);
                return None;
            }
            Ok(main) => {
                return Some(main);
            }
        }
    }
}

pub fn update_cv_main_sort(
    request: &Json<EditMainSort>,
    login_user_info: &LoginUserInfo,
) -> Option<CvMain> {
    use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_main::id
        .eq(request.id)
        .and(crate::model::diesel::cv::cv_schema::cv_main::user_id.eq(login_user_info.userId));
    let update_result = diesel::update(cv_main.filter(predicate))
        .set(item_order.eq(&request.item_order))
        .get_result::<CvMain>(&mut get_connection())
        .expect("unable to update cv order");
    return Some(update_result);
}

pub fn copy_cv_main(request: &Json<CopyMainCv>, login_user_info: &LoginUserInfo) -> Option<i64> {
    let mut connection = get_connection();
    let cv_resp = get_cv_by_id(request.id, login_user_info);
    match cv_resp {
        Some(main) => {
            let mut cv_summary = CvMainAdd::from_resp(&main, login_user_info);
            cv_summary.cv_name = format!("{}{}", cv_summary.cv_name, "-Copy");
            use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
            let _result = connection.transaction(|conn| {
                let record_id = diesel::insert_into(cv_main)
                    .values(&cv_summary)
                    .returning(id)
                    .get_result(conn);
                match record_id {
                    Ok(inserted_cv_id) => {
                        insert_edu(main.edu, login_user_info, &inserted_cv_id);
                        insert_work(main.work, login_user_info, &inserted_cv_id);
                        insert_skills(main.skills, login_user_info, &inserted_cv_id);
                        insert_project(main.projects, login_user_info, &inserted_cv_id);
                        insert_langs(main.langs, login_user_info, &inserted_cv_id);
                    }
                    Err(_) => {}
                }
                return record_id;
            });
            return Some(1);
        }
        None => {}
    }
    return Some(1);
}

fn insert_edu(edues: Vec<CvEduResp>, login_user_info: &LoginUserInfo, inserted_cv_id: &i64) {
    use crate::model::diesel::cv::cv_schema::cv_edu::dsl::*;
    for edu in edues.iter() {
        let mut edu_add = CvEduAdd::from_edu_resp(edu, login_user_info);
        edu_add.cv_id = inserted_cv_id.clone();
        let _result = diesel::insert_into(cv_edu)
            .values(&edu_add)
            .get_result::<CvEdu>(&mut get_connection());
    }
}

fn insert_work(workes: Vec<CvWorkResp>, login_user_info: &LoginUserInfo, inserted_cv_id: &i64) {
    use crate::model::diesel::cv::cv_schema::cv_work_exp::dsl::*;
    for edu in workes.iter() {
        let mut edu_add = CvWorkExpAdd::from_work_resp(edu, login_user_info);
        edu_add.cv_id = inserted_cv_id.clone();
        let _result = diesel::insert_into(cv_work_exp)
            .values(&edu_add)
            .get_result::<CvWorkExp>(&mut get_connection());
    }
}

fn insert_skills(workes: Vec<CvSkillResp>, login_user_info: &LoginUserInfo, inserted_cv_id: &i64) {
    use crate::model::diesel::cv::cv_schema::cv_skills::dsl::*;
    for edu in workes.iter() {
        let mut edu_add = CvSkillAdd::from_work_resp(edu, login_user_info);
        edu_add.cv_id = inserted_cv_id.clone();
        let _result = diesel::insert_into(cv_skills)
            .values(&edu_add)
            .get_result::<CvSkill>(&mut get_connection());
    }
}

fn insert_langs(workes: Vec<CvLangResp>, login_user_info: &LoginUserInfo, inserted_cv_id: &i64) {
    use crate::model::diesel::cv::cv_schema::cv_lang::dsl::*;
    for edu in workes.iter() {
        let mut edu_add = CvLangAdd::from_work_resp(edu, login_user_info);
        edu_add.cv_id = inserted_cv_id.clone();
        let _result = diesel::insert_into(cv_lang)
            .values(&edu_add)
            .get_result::<CvLang>(&mut get_connection());
    }
}

fn insert_project(
    workes: Vec<CvProjectResp>,
    login_user_info: &LoginUserInfo,
    inserted_cv_id: &i64,
) {
    use crate::model::diesel::cv::cv_schema::cv_project_exp::dsl::*;
    for edu in workes.iter() {
        let mut edu_add = CvProjectExpAdd::from_work_resp(edu, login_user_info);
        edu_add.cv_id = inserted_cv_id.clone();
        let _result = diesel::insert_into(cv_project_exp)
            .values(&edu_add)
            .get_result::<CvProjectExp>(&mut get_connection());
    }
}

pub fn update_cv_template(cv_id: &i64, tpl_id: &i64, login_user_info: &LoginUserInfo) -> Option<CvTemplate> {
    use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_main::id
        .eq(cv_id)
        .and(crate::model::diesel::cv::cv_schema::cv_main::user_id.eq(login_user_info.userId));
    let update_result = diesel::update(cv_main.filter(predicate))
        .set(template_id.eq(tpl_id))
        .get_result::<CvMain>(&mut get_connection())
        .expect("unable to update cv main");
    let tpl_result = get_tempalte_by_id(update_result.template_id);
    return tpl_result;
}

pub fn update_cv_main_color(request: &Json<UpdateMainCvColor>, login_user_info: &LoginUserInfo) -> CvMain {
    use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_main::id
        .eq(request.cv_id)
        .and(crate::model::diesel::cv::cv_schema::cv_main::user_id.eq(login_user_info.userId));
    let update_result = diesel::update(cv_main.filter(predicate))
        .set(main_color.eq(request.main_color.clone()))
        .get_result::<CvMain>(&mut get_connection())
        .expect("unable to update cv main color");
    return update_result;
}

pub fn update_cv_main_config(request: &Json<UpdateMainCvConfig>, login_user_info: &LoginUserInfo) -> CvMain {
    #[derive(AsChangeset)]
    #[diesel(table_name = crate::model::diesel::cv::cv_schema::cv_main)]
    struct PostForm<'a> {
        theme: Option<&'a str>,
        font_size: Option<&'a str>,
    }
    use crate::model::diesel::cv::cv_schema::cv_main::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_main::id
        .eq(request.cv_id)
        .and(crate::model::diesel::cv::cv_schema::cv_main::user_id.eq(login_user_info.userId));
    let update_result = diesel::update(cv_main.filter(predicate))
        .set(&PostForm{
            theme: request.theme.as_deref(),
            font_size: request.font_size.as_deref(),
})
        .get_result::<CvMain>(&mut get_connection())
        .expect("unable to update cv main theme");
    return update_result;
}
