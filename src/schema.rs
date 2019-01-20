table! {
    questions (id) {
        id -> Int4,
        survey_id -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
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

joinable!(questions -> surveys (survey_id));
joinable!(surveys -> users (author));

allow_tables_to_appear_in_same_query!(
    questions,
    surveys,
    users,
);
