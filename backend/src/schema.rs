// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Uuid,
        name -> Varchar,
        user_id -> Uuid,
        color -> Varchar,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        name -> Varchar,
        user_id -> Uuid,
        project_id -> Uuid,
        color -> Varchar,
    }
}

diesel::table! {
    trackings (id) {
        id -> Uuid,
        user_id -> Uuid,
        project_id -> Uuid,
        task_id -> Nullable<Uuid>,
        start -> Timestamptz,
        end -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
    }
}

diesel::joinable!(trackings -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
    trackings,
    users,
);
