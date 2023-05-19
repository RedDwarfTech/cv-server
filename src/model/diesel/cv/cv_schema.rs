table! {
    cv_gen (id) {
        id -> Int8,
        cv_name -> Varchar,
        remark -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Int8,
        cv_status -> Int4,
        gen_time -> Nullable<Int8>,
        path -> Nullable<Varchar>,
    }
}

table! {
    cv_template (id) {
        id -> Int8,
        name -> Varchar,
        remark -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        template_status -> Int4,
        template_id -> Int8,
        preview_url -> Nullable<Varchar>,
        template_code -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    cv_gen,
    cv_template,
);
