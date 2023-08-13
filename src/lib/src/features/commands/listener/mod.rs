use crate::extensions::serenity::Command;

mod listen;
mod privacy;

pub fn group() -> Vec<Command> {
    vec![listen::listen(), privacy::privacy()]
}
