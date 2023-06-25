use crate::extensions::serenity::Command;

mod att;
pub mod deploy;
mod stats;

pub fn group<'a>() -> Vec<Command> {
    vec![att::att(), stats::stats(), deploy::deploy()]
}
