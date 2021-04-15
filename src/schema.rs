table! {
    banlist (user_id) {
        user_id -> Text,
        user_handle -> Nullable<Text>,
        ban_date -> Text,
    }
}
