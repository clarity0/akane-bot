table! {
    banlist (user_id) {
        user_id -> Varchar,
        server_id -> Varchar,
        user_handle -> Varchar,
        ban_date -> Text,
    }
}

table! {
    mutelist (user_id) {
        user_id -> Varchar,
        server_id -> Varchar,
        user_handle -> Varchar,
        mute_date -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    banlist,
    mutelist,
);
