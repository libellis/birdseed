table! {
    use diesel::sql_types::*;

    categories (title) {
        title -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    choices (id) {
        id -> Int4,
        question_id -> Int4,
        content -> Nullable<Text>,
        content_type -> Text,
        title -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    fences (title) {
        title -> Text,
        geo_level -> Int4,
        geo -> Geography,
    }
}

table! {
    use diesel::sql_types::*;

    questions (id) {
        id -> Int4,
        survey_id -> Int4,
        question_type -> Text,
        title -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;

    surveys (id) {
        id -> Int4,
        author -> Text,
        title -> Text,
        description -> Nullable<Text>,
        anonymous -> Bool,
        published -> Bool,
        date_posted -> Timestamp,
        category -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    users (username) {
        username -> Text,
        password -> Text,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        photo_url -> Nullable<Text>,
        is_admin -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    votes (choice_id, username) {
        choice_id -> Int4,
        username -> Text,
        score -> Int4,
        geo -> Geography,
        fence_title -> Text,
        date_voted -> Timestamp,
    }
}

joinable!(choices -> questions (question_id));
joinable!(questions -> surveys (survey_id));
joinable!(surveys -> categories (category));
joinable!(surveys -> users (author));
joinable!(votes -> choices (choice_id));
joinable!(votes -> fences (fence_title));
joinable!(votes -> users (username));

allow_tables_to_appear_in_same_query!(
    categories,
    choices,
    fences,
    questions,
    spatial_ref_sys,
    surveys,
    users,
    votes,
);
