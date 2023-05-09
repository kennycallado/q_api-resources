// @generated automatically by Diesel CLI.

diesel::table! {
    resource_module (id) {
        id -> Int4,
        resource_id -> Int4,
        slide_id -> Int4,
    }
}

diesel::table! {
    resource_slides (id) {
        id -> Int4,
        resource_id -> Int4,
        slide_id -> Int4,
    }
}

diesel::table! {
    resources (id) {
        id -> Int4,
        resource_type -> Varchar,
        title -> Varchar,
        description -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(resource_module -> resources (resource_id));
diesel::joinable!(resource_slides -> resources (resource_id));

diesel::allow_tables_to_appear_in_same_query!(
    resource_module,
    resource_slides,
    resources,
);
