use crate::error::Error;
use crate::models::insertable::NewGulag;
use crate::models::queryable::Gulag;
use crate::schema::gulags;
use crate::{database::establish_connection, log_add, log_remove};
use diesel::prelude::*;
use serenity::model::{guild::Guild, prelude::User};

log_add!(log_gulag, gulags, Gulag, NewGulag);

log_remove!(log_ungulag, gulags, Gulag);
