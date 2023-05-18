use diesel::{QueryDsl, ExpressionMethods, TextExpressionMethods};
use rust_wheel::{model::user::login_user_info::LoginUserInfo};
use rust_wheel::config::db::config;
use crate::model::diesel::cv::cv_models::CvGen;
use crate::diesel::RunQueryDsl;

pub fn cv_gen_list(filter_name: Option<String>,login_user_info: &LoginUserInfo) -> Vec<CvGen> {
    let mut connection = config::connection("CV_DATABASE_URL".to_string());
    use crate::model::diesel::cv::cv_schema::cv_gen as cv_gen_table;
    let mut query = cv_gen_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(cv_gen_table::cv_name.like(format!("{}{}{}","%",some_filter_name.as_str(),"%")));
    }
    query = query.filter(cv_gen_table::user_id.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<CvGen>(&mut connection)
        .expect("error get user bill book");
    return user_bill_books;
}


