table! {
    cv_edu (id) {
        id -> Int8,
        edu_addr -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        cv_id -> Int8,
        degree -> Nullable<Varchar>,
        major -> Nullable<Varchar>,
        admission -> Nullable<Date>,
        graduation -> Nullable<Date>,
        user_id -> Int8,
    }
}

table! {
    cv_gen (id) {
        id -> Int8,
        cv_name -> Varchar,
        remark -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Int8,
        gen_status -> Int4,
        gen_time -> Nullable<Int8>,
        path -> Nullable<Varchar>,
        template_id -> Int8,
        cv_id -> Int8,
    }
}

table! {
    cv_main (id) {
        id -> Int8,
        cv_name -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Int8,
        cv_status -> Int4,
        template_id -> Int8,
        employee_name -> Nullable<Varchar>,
        birthday -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        job -> Nullable<Varchar>,
        workplace -> Nullable<Varchar>,
        stackoverflow -> Nullable<Varchar>,
        github -> Nullable<Varchar>,
        blog -> Nullable<Varchar>,
    }
}

table! {
    cv_section (id) {
        id -> Int8,
        section_abbr -> Varchar,
        remark -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        cv_id -> Nullable<Int8>,
    }
}

table! {
    cv_section_content (id) {
        id -> Int8,
        section_id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        item_key -> Nullable<Varchar>,
        item_value -> Nullable<Varchar>,
    }
}

table! {
    cv_section_type (id) {
        id -> Int8,
        item_name -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        item_abbr -> Nullable<Varchar>,
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

table! {
    cv_work_exp (id) {
        id -> Int8,
        company -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        cv_id -> Int8,
        job -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        work_start -> Nullable<Date>,
        work_end -> Nullable<Date>,
        user_id -> Int8,
        duty -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    cv_edu,
    cv_gen,
    cv_main,
    cv_section,
    cv_section_content,
    cv_section_type,
    cv_template,
    cv_work_exp,
);
