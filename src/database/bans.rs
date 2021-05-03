use crate::error::Error;
use crate::models::insertable::NewBan;
use crate::models::queryable::Ban;
use crate::schema::bans;
use crate::{database::establish_connection, log_add, log_remove};
use diesel::prelude::*;
use serenity::model::{guild::Guild, prelude::User};

log_add!(log_ban, bans, Ban, NewBan);

log_remove!(log_unban, bans, Ban);
