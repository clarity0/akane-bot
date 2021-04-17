table! {
    banlist (user_id) {
        user_id -> Varchar,
        server_id -> Varchar,
        user_handle -> Varchar,
        ban_date -> Text,
    }
}
