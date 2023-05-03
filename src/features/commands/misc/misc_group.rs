use serenity::framework::standard::macros::group;
use crate::features::commands::misc::{att::ATT_COMMAND, stats::STATS_COMMAND, debug::DEBUG_COMMAND};

#[group]
#[commands(att, stats, debug)]
#[summary = "Miscellaneous"]
#[description = "Simplesmente comandos miscellaneous..."]
pub struct Misc;