use crate::features::commands::r34::random::RANDOM_COMMAND;
use serenity::framework::standard::macros::group;

#[group]
#[commands(random)]
#[summary = "Rule 34"]
#[description = "Comandos usando a api do rule 34"]
pub struct R34;
