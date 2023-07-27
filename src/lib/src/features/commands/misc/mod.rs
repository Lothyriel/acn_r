use crate::extensions::serenity::Command;

mod att;
pub mod deploy;
pub mod privacy;

pub fn group() -> Vec<Command> {
    vec![att::att(), deploy::deploy()]
}
