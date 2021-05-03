use crate::{database::establish_connection, log_add, log_remove};
use crate::error::Error;
use crate::models::insertable::NewMute;
use crate::models::queryable::Mute;
use crate::schema::mutes;
use diesel::prelude::*;
use serenity::model::{guild::Guild, prelude::User};

log_add!(log_mute, mutes, Mute, NewMute);

log_remove!(log_unmute, mutes, Mute);
