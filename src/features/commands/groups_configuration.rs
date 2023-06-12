use crate::{
    extensions::serenity::serenity_structs::Command,
    features::commands::{help::help, misc::misc_group::misc_group, r34::r34_group::r34_group, reactions::reactions_group::reactions_group},
};

fn register_groups() -> Vec<Vec<Command>> {
    vec![r34_group(), misc_group(), reactions_group()]
}

pub fn register_commands() -> Vec<Command> {
    let mut commands = vec![help()];
    for mut command in register_groups() {
        commands.append(command.as_mut());
    }
    commands
}
