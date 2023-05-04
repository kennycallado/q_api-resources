// @generated automatically by Diesel CLI.

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
    }
}

diesel::joinable!(resource_slides -> resources (resource_id));

diesel::allow_tables_to_appear_in_same_query!(resource_slides, resources,);
