table! {
    request_logs (id) {
        id -> Uuid,
        url -> Text,
        headers -> Text,
        response_code -> Nullable<Int4>,
        response_size_bytes -> Nullable<Int4>,
        created_on -> Timestamptz,
        finished_on -> Nullable<Timestamptz>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        login_name -> Text,
        password -> Text,
        email -> Text,
    }
}

table! {
    user_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_on -> Timestamptz,
        ip -> Text,
    }
}

joinable!(user_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    request_logs,
    users,
    user_tokens,
);
