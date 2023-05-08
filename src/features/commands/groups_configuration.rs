use crate::{
    extensions::serenity_ext::Command,
    features::commands::{misc::misc_group::misc_group, r34::r34_group::r34_group},
};

fn register_groups() -> Vec<Vec<Command>> {
    vec![r34_group()]
}

pub fn register_commands() -> Vec<Command> {
    let mut commands = Vec::new();
    for mut command in register_groups() {
        commands.append(command.as_mut());
    }
    commands
}
