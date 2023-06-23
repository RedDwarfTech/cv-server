use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvSkill, CvLang};
use crate::model::orm::cv::lang::cv_lang_add::CvLangAdd;
use crate::model::request::cv::lang::langs_request::LangsRequest;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn add_lang(
    request: &Json<LangsRequest>,
    login_user_info: &LoginUserInfo,
) -> Result<CvLang, diesel::result::Error> {
    use crate::model::diesel::cv::cv_schema::cv_lang::dsl::*;
    let cv_edu_model = CvLangAdd::from_req(request, login_user_info);
    if request.id.is_some() {
        let predicate = crate::model::diesel::cv::cv_schema::cv_lang::id.eq(request.id.unwrap());
        let update_result = diesel::update(cv_lang.filter(predicate))
            .set((
                updated_time.eq(get_current_millisecond()),
                name.eq(request.name.clone()),
                level.eq(request.level.clone()),
            ))
            .get_result::<CvLang>(&mut get_connection());
        return update_result;
    } else {
        let result = diesel::insert_into(cv_lang)
            .values(&cv_edu_model)
            .on_conflict(id)
            .do_update()
            .set((
                updated_time.eq(get_current_millisecond()),
                name.eq(request.name.clone()),
                level.eq(request.level.clone()),
            ))
            .get_result::<CvLang>(&mut get_connection());
        return result;
    }
}

pub fn get_ui_lang_list(cv_id: &i64, login_user_info: &LoginUserInfo) -> Vec<CvSkill> {
    use crate::model::diesel::cv::cv_schema::cv_skills as cv_work_table;
    let mut query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_work_table::user_id.eq(login_user_info.userId));
    query = query.filter(cv_work_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvSkill>(&mut get_connection())
        .expect("error get work list");
    return cvs;
}

pub fn _get_lang_list(cv_id: &i64) -> Vec<CvSkill> {
    use crate::model::diesel::cv::cv_schema::cv_skills as cv_work_table;
    let mut query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_work_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvSkill>(&mut get_connection())
        .expect("error get work list");
    return cvs;
}

pub fn del_lang_item(item_id: &i64, login_user_info: &LoginUserInfo) -> bool {
    use crate::model::diesel::cv::cv_schema::cv_skills::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_skills::id
        .eq(item_id)
        .and(crate::model::diesel::cv::cv_schema::cv_skills::user_id.eq(login_user_info.userId));
    let delete_result = diesel::delete(cv_skills.filter(predicate)).execute(&mut get_connection());
    match delete_result {
        Ok(_v) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

pub fn _del_langs_items(del_cv_id: &i64, login_user_info: &LoginUserInfo) -> bool {
    use crate::model::diesel::cv::cv_schema::cv_skills::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_skills::cv_id
        .eq(del_cv_id)
        .and(crate::model::diesel::cv::cv_schema::cv_skills::user_id.eq(login_user_info.userId));
    let delete_result = diesel::delete(cv_skills.filter(predicate)).execute(&mut get_connection());
    match delete_result {
        Ok(_v) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}
