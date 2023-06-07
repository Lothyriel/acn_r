use crate::{
    extensions::serenity::serenity_structs::Command,
    features::commands::misc::{att::att, deploy::deploy, stats::stats},
};

pub fn misc_group<'a>() -> Vec<Command> {
    vec![att(), stats(), deploy()]
}
