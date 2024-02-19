use crate::extensions::serenity::Command;

mod att;
mod signature;

pub fn group() -> Vec<Command> {
    vec![att::att(), signature::set_signature()]
}
