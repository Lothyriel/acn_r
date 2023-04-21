use serenity::framework::standard::macros::group;
use crate::features::commands::misc::att::ATT_COMMAND;

#[group]
#[commands(att)]
#[summary = "Miscellaneous"]
#[description = "Simplesmente comandos miscellaneous..."]
pub struct Misc;