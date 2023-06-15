use crate::{
    extensions::serenity::serenity_structs::Command,
    features::commands::{help::help, jukebox::jukebox_group, misc::misc_group, r34::r34_group},
};

fn register_groups() -> Vec<Vec<Command>> {
    vec![r34_group(), misc_group(), jukebox_group()]
}

pub fn register_commands() -> Vec<Command> {
    let mut commands = vec![help()];

    for mut command in register_groups() {
        commands.append(command.as_mut());
    }

    commands
}
