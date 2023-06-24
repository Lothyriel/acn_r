use crate::{
    extensions::serenity::Command,
    features::commands::{help::help, jukebox, misc, r34, reactions},
};

fn register_groups() -> Vec<Vec<Command>> {
    vec![
        r34::group(),
        misc::group(),
        jukebox::group(),
        reactions::group(),
    ]
}

pub fn register_commands() -> Vec<Command> {
    let mut commands = vec![help()];

    for mut command in register_groups() {
        commands.append(command.as_mut());
    }

    commands
}
