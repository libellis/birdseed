table! {
    choices (id) {
        id -> Int4,
        question_id -> Int4,
        content -> Nullable<Text>,
        content_type -> Text,
        title -> Text,
    }
}

table! {
    questions (id) {
        id -> Int4,
        survey_id -> Int4,
        question_type -> Text,
        title -> Text,
    }
}

table! {
    surveys (id) {
        id -> Int4,
        author -> Text,
        title -> Text,
        description -> Nullable<Text>,
        anonymous -> Bool,
        published -> Bool,
        date_posted -> Timestamp,
    }
}

table! {
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
    votes (choice_id, username) {
        choice_id -> Int4,
        username -> Text,
        score -> Int4,
    }
}

joinable!(choices -> questions (question_id));
joinable!(questions -> surveys (survey_id));
joinable!(surveys -> users (author));
joinable!(votes -> choices (choice_id));
joinable!(votes -> users (username));

allow_tables_to_appear_in_same_query!(
    choices,
    questions,
    surveys,
    users,
    votes,
);
