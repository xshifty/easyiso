// @generated automatically by Diesel CLI.

diesel::table! {
    certification_checklists (certification_id, checklist_id) {
        certification_id -> Int4,
        checklist_id -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    certification_events (certification_id, event_id) {
        certification_id -> Int4,
        event_id -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    certification_roles (certification_id, role_id) {
        certification_id -> Int4,
        role_id -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    certifications (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    checklist_items (id) {
        id -> Int4,
        checklist_id -> Nullable<Int4>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 500]
        description -> Varchar,
        checklist_order -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    checklists (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    event_logs (id) {
        id -> Int4,
        event_id -> Nullable<Int4>,
        event_start -> Timestamp,
        event_end -> Timestamp,
        event_result -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    event_schedules (id) {
        id -> Int4,
        event_id -> Nullable<Int4>,
        event_start -> Timestamp,
        event_end -> Nullable<Timestamp>,
        event_count -> Int4,
        event_delay_sec -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    group_roles (group_id, role_id) {
        group_id -> Int4,
        role_id -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_groups (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 150]
        full_name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 256]
        password -> Nullable<Varchar>,
        enabled -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(certification_checklists -> certifications (certification_id));
diesel::joinable!(certification_checklists -> checklists (checklist_id));
diesel::joinable!(certification_events -> certifications (certification_id));
diesel::joinable!(certification_events -> events (event_id));
diesel::joinable!(certification_roles -> certifications (certification_id));
diesel::joinable!(certification_roles -> roles (role_id));
diesel::joinable!(checklist_items -> checklists (checklist_id));
diesel::joinable!(event_logs -> events (event_id));
diesel::joinable!(event_schedules -> events (event_id));
diesel::joinable!(group_roles -> groups (group_id));
diesel::joinable!(group_roles -> roles (role_id));
diesel::joinable!(user_groups -> groups (group_id));
diesel::joinable!(user_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    certification_checklists,
    certification_events,
    certification_roles,
    certifications,
    checklist_items,
    checklists,
    event_logs,
    event_schedules,
    events,
    group_roles,
    groups,
    roles,
    user_groups,
    users,
);
