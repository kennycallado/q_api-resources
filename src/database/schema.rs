// @generated automatically by Diesel CLI.

diesel::table! {
    resource_form (id) {
        id -> Int4,
        resource_id -> Int4,
        question_id -> Int4,
    }
}

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
        sort_order -> Array<Nullable<Int4>>,
        title -> Varchar,
        description -> Varchar,
    }
}

diesel::joinable!(resource_form -> resources (resource_id));
diesel::joinable!(resource_module -> resources (resource_id));
diesel::joinable!(resource_slides -> resources (resource_id));

diesel::allow_tables_to_appear_in_same_query!(
    resource_form,
    resource_module,
    resource_slides,
    resources,
);
