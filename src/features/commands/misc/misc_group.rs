use crate::{
    extensions::serenity_ext::Command,
    features::commands::misc::{att::att, deploy::deploy, stats::stats},
};

pub fn misc_group<'a>() -> Vec<Command> {
    vec![att(), stats(), deploy()]
}
