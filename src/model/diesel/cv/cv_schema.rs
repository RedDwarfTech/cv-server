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
