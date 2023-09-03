use std::env::join_paths;
use std::error::Error;
use std::io::{BufRead, BufReader};

use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::cv::custom_cv_models::{CvGen, CvGenAdd, CvGenUpdate};
use crate::model::request::cv::gen_request::GenRequest;
use crate::model::request::cv::render_result_request::RenderResultRequest;
use crate::model::response::cvgen::cv_gen_resp::CvGenResp;
use crate::service::template::template_service::get_tempalte_list;
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, TextExpressionMethods,
};
use log::error;
use rocket::serde::json::Json;
use rust_wheel::common::error::not_vip_error::NotVipError;
use rust_wheel::common::util::model_convert::map_entity;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::app::app_conf_reader::get_app_config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use std::fs::{self, File};

pub fn cv_gen_list(filter_name: Option<String>, login_user_info: &LoginUserInfo) -> Vec<CvGenResp> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!(
            "{}{}{}",
            "%",
            some_filter_name.as_str(),
            "%"
        )));
    }
    query = query
        .filter(cv_gen_table::user_id.eq(login_user_info.userId))
        .order(cv_gen_table::created_time.desc());
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get user gen record");
    let mut gen_resp: Vec<CvGenResp> = map_entity(user_bill_books);
    if !gen_resp.is_empty() {
        let ids: Vec<i64> = gen_resp.iter().map(|part| part.template_id).collect();
        let templtes = get_tempalte_list(Some(ids));
        for resp in gen_resp.iter_mut() {
            let tpl = templtes
                .iter()
                .find(|tpl| tpl.template_id == resp.template_id);
            resp.template_name = Some(tpl.as_deref().unwrap().name.clone());
            resp.preview_url = tpl.as_deref().unwrap().preview_url.clone();
        }
    }
    return gen_resp;
}

pub fn get_gen_by_id(gen_id: i64, login_user_info: &LoginUserInfo) -> CvGen {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(
        cv_gen_table::user_id
            .eq(login_user_info.userId)
            .and(cv_gen_table::id.eq(gen_id)),
    );
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get user gen record");
    return user_bill_books[0].clone();
}

pub fn pick_task() -> Result<Option<CvGen>, diesel::result::Error> {
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let mut connection = get_connection();
    let result = connection.transaction(|connection| {
        use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
        let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
        query = query.filter(cv_gen_table::gen_status.eq(0)).limit(1);
        let user_bill_books = query
            .load::<CvGen>(connection)
            .expect("error get cv gen record");
        if user_bill_books.is_empty() {
            return Ok(None);
        }
        let updated_rows = diesel::update(cv_gen.find(user_bill_books[0].id))
            .set(gen_status.eq(1))
            .get_result::<CvGen>(connection);
        match updated_rows {
            Ok(r) => Ok(Some(r)),
            Err(e) => diesel::result::QueryResult::Err(e),
        }
    });
    return result;
}

pub fn cv_gen_page(filter_name: Option<String>, login_user_info: &LoginUserInfo) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!(
            "{}{}{}",
            "%",
            some_filter_name.as_str(),
            "%"
        )));
    }
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get user gen record");
    return user_bill_books;
}

pub fn check_paied_plan(
    request: &Json<GenRequest>,
    login_user_info: &LoginUserInfo,
) -> Result<CvGen, Box<dyn Error>> {
    if login_user_info.vipExpireTime <= get_current_millisecond() {
        return Err(Box::new(NotVipError::new("vip-expired".to_owned(), None)));
    }
    return create_gen_task(request, login_user_info);
}

pub fn create_gen_task(
    request: &Json<GenRequest>,
    login_user_info: &LoginUserInfo,
) -> Result<CvGen, Box<dyn Error>> {
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let cv_gen_add = CvGenAdd {
        cv_name: request.cv_name.to_owned(),
        remark: "".to_owned(),
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        user_id: login_user_info.userId,
        gen_status: 0,
        gen_time: Some(0),
        path: Some("".to_owned()),
        template_id: request.template_id,
        cv_id: request.cv_id,
    };
    let result = diesel::insert_into(cv_gen)
        .values(&cv_gen_add)
        .on_conflict(id)
        .do_update()
        .set(((updated_time.eq(get_current_millisecond())),))
        .get_result::<CvGen>(&mut get_connection());
    return Ok(result?);
}

pub fn cv_gen_list_render(filter_name: Option<String>) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!(
            "{}{}{}",
            "%",
            some_filter_name.as_str(),
            "%"
        )));
    }
    query = query.filter(cv_gen_table::gen_status.eq(0));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get render gen record");
    return user_bill_books;
}

pub fn update_gen_result(request: Json<RenderResultRequest>) {
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let predicate = crate::model::diesel::cv::cv_schema::cv_gen::id.eq(request.id);
    diesel::update(cv_gen.filter(predicate))
        .set(&CvGenUpdate {
            gen_status: request.gen_status,
            path: request.path.clone(),
            tex_file_path: request.tex_file_path.clone(),
            gen_time: Some(get_current_millisecond()),
            updated_time: get_current_millisecond(),
        })
        .get_result::<CvGen>(&mut get_connection())
        .expect("unable to update ren result");
}

pub fn check_gen_status(ids: String, login_user_info: &LoginUserInfo) -> Vec<CvGen> {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let id_array: Vec<i64> = ids.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_gen_table::id.eq_any(id_array));
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get render gen record");
    return user_bill_books;
}

pub fn get_cv_src(gid: i64, login_user_info: &LoginUserInfo) -> String {
    let gen = get_gen_by_id(gid, login_user_info);
    if gen.tex_file_path.is_none() {
        return "".to_owned();
    }
    let base_cv_dir = get_app_config("cv.cv_compile_base_dir");
    let file_path = join_paths(&[base_cv_dir,gen.tex_file_path.unwrap().clone()]);
    if let Err(e) = file_path {
        error!("join path failed, {}", e);
        return "".to_owned();
    }
    let file = File::open(file_path.unwrap());
    match file {
        Ok(read_file) => {
            let reader = BufReader::new(read_file);
            let mut tex_content = String::new();
            for line in reader.lines() {
                let line = line.unwrap_or_default();
                tex_content.push_str(&line);
                tex_content.push('\n');
            }
            return tex_content;
        }
        Err(e) => {
            error!("read file facing error, {}, gid: {}", e, gid);
            return "".to_owned();
        }
    }
}

pub fn del_gen_impl(gen_id: &i64, login_user_info: &LoginUserInfo) -> bool {
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    use crate::model::diesel::cv::cv_schema::cv_gen::dsl::*;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_gen_table::id.eq(gen_id));
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let gen_record: Vec<CvGen> = query
        .load::<CvGen>(&mut get_connection())
        .expect("error get render gen record");
    if gen_record.is_empty() {
        error!("pass invalid id when delete gen, id: {}", gen_id);
        return false;
    }
    let predicate = crate::model::diesel::cv::cv_schema::cv_gen::id
        .eq(gen_id)
        .and(crate::model::diesel::cv::cv_schema::cv_gen::user_id.eq(login_user_info.userId));
    let delete_result: Result<usize, diesel::result::Error> =
        diesel::delete(cv_gen.filter(predicate)).execute(&mut get_connection());
    match delete_result {
        Ok(_v) => {
            let unwrap_record = gen_record.get(0).unwrap();
            let file_name = unwrap_record.path.as_ref().unwrap();
            let pdf_file_path = format!("{}{}", "/opt/cv/pdf/", file_name);
            match fs::remove_file(pdf_file_path) {
                Ok(_) => {}
                Err(e) => error!("delete cv pdf failed:{}", e),
            }
            return true;
        }
        Err(e) => {
            error!("delete gen record facing error: {}", e);
            return false;
        }
    }
}
