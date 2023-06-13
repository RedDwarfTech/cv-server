use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::CvSkill;
use crate::model::orm::cv::skill::cv_skill_add::CvSkillAdd;
use crate::model::request::cv::skills::skills_request::SkillsRequest;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

pub fn add_skill(
    request: &Json<SkillsRequest>,
    login_user_info: &LoginUserInfo,
) -> Result<CvSkill, diesel::result::Error> {
    use crate::model::diesel::cv::cv_schema::cv_skills::dsl::*;
    let cv_edu_model = CvSkillAdd {
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        cv_id: request.cv_id,
        user_id: login_user_info.userId,
        name: request.name.clone(),
        level: request.level.clone(),
        memo: request.memo.clone(),
    };

    if request.id.is_some() {
        let predicate = crate::model::diesel::cv::cv_schema::cv_skills::id.eq(request.id.unwrap());
        let update_result = diesel::update(cv_skills.filter(predicate))
            .set((
                updated_time.eq(get_current_millisecond()),
                name.eq(request.name.clone()),
                level.eq(request.level.clone()),
            ))
            .get_result::<CvSkill>(&mut get_connection());
        return update_result;
    } else {
        let result = diesel::insert_into(cv_skills)
            .values(&cv_edu_model)
            .on_conflict(id)
            .do_update()
            .set((
                updated_time.eq(get_current_millisecond()),
                name.eq(request.name.clone()),
                level.eq(request.level.clone()),
            ))
            .get_result::<CvSkill>(&mut get_connection());
        return result;
    }
}

pub fn get_ui_skill_list(cv_id: &i64, login_user_info: &LoginUserInfo) -> Vec<CvSkill> {
    use crate::model::diesel::cv::cv_schema::cv_skills as cv_work_table;
    let mut query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_work_table::user_id.eq(login_user_info.userId));
    query = query.filter(cv_work_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvSkill>(&mut get_connection())
        .expect("error get work list");
    return cvs;
}

pub fn get_skill_list(cv_id: &i64) -> Vec<CvSkill> {
    use crate::model::diesel::cv::cv_schema::cv_skills as cv_work_table;
    let mut query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_work_table::cv_id.eq(cv_id));
    let cvs = query
        .load::<CvSkill>(&mut get_connection())
        .expect("error get work list");
    return cvs;
}

pub fn del_skill_item(item_id: &i64, login_user_info: &LoginUserInfo) -> bool {
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

pub fn del_skills_items(del_cv_id: &i64, login_user_info: &LoginUserInfo) -> bool {
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
