use crate::extensions::serenity::Command;

mod att;
pub mod deploy;
mod signature;

pub fn group() -> Vec<Command> {
    vec![att::att(), deploy::deploy(), signature::set_signature()]
}
