use crate::extensions::serenity::serenity_structs::Command;

mod att;
mod deploy;
mod stats;

pub fn misc_group<'a>() -> Vec<Command> {
    vec![att::att(), stats::stats(), deploy::deploy()]
}
