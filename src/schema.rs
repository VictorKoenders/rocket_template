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
        register_request_id -> Uuid,
        name -> Text,
        login_name -> Text,
        password -> Text,
        email -> Text,
        email_confirmed_request_id -> Nullable<Uuid>,
    }
}

table! {
    user_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_on -> Timestamptz,
        created_request_id -> Uuid,
        ip -> Text,
    }
}

joinable!(user_tokens -> request_logs (created_request_id));
joinable!(user_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    request_logs,
    users,
    user_tokens,
);
