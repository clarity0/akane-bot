table! {
    banlist (user_id) {
        user_id -> Varchar,
        server_id -> Varchar,
        user_handle -> Varchar,
        date -> Timestamp,
    }
}

table! {
    mutelist (user_id) {
        user_id -> Varchar,
        server_id -> Varchar,
        user_handle -> Varchar,
        date -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    banlist,
    mutelist,
);
