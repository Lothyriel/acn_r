use crate::{features::commands::reactions::{react::react, add_react::add_react}, extensions::serenity::serenity_structs::Command};

pub fn reactions_group() -> Vec<Command> {
    vec![react(), add_react()]
}