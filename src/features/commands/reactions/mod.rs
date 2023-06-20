use crate::extensions::serenity::serenity_structs::Command;

mod add_react;
mod react;

pub fn group() -> Vec<Command> {
    vec![react::react(), add_react::add_react()]
}
