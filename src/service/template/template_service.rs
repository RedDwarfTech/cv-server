use diesel::{QueryDsl, ExpressionMethods};
use crate::common::database::get_connection;
use crate::model::diesel::cv::custom_cv_models::CvTemplate;
use crate::diesel::RunQueryDsl;

pub fn get_tempalte_by_id(tpl_id: i64) -> CvTemplate {
    use crate::model::diesel::cv::cv_schema::cv_template as cv_tpl_table;
    let mut query = cv_tpl_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_tpl_table::template_id.eq(tpl_id));
    let tpl = query
        .load::<CvTemplate>(&mut get_connection())
        .expect("error get template by id");
    return tpl.get(0).unwrap().to_owned();
}

pub fn get_tempalte_list(ids: Option<Vec<i64>>) -> Vec<CvTemplate> {
    use crate::model::diesel::cv::cv_schema::cv_template as cv_tpl_table;
    let mut query = cv_tpl_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(ids) = ids {
        query = query.filter(cv_tpl_table::template_id.eq_any(ids));
    }
    let tpl = query
        .load::<CvTemplate>(&mut get_connection())
        .expect("error get cv template list");
    return tpl;
}

pub fn get_online_tempalte_list(ids: Option<Vec<i64>>) -> Vec<CvTemplate> {
    use crate::model::diesel::cv::cv_schema::cv_template as cv_tpl_table;
    let mut query = cv_tpl_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(ids) = ids {
        query = query.filter(cv_tpl_table::template_id.eq_any(ids));
    }
    query = query.filter(cv_tpl_table::online_status.eq(1));
    let tpl = query
        .load::<CvTemplate>(&mut get_connection())
        .expect("error get cv template list");
    return tpl;
}