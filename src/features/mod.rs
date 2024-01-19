use crate::{
    extensions::{log_ext::LogExt, serenity::Command},
    features::commands::help,
};

mod acn;
mod commands;
mod events;
mod listener;

pub async fn start_acn() {
    acn::start_acn().await.log();
}

pub async fn start_listener() {
    listener::start_listener().await.log();
}

fn register_commands(groups: Vec<Vec<Command>>) -> Vec<Command> {
    let mut commands = vec![help::help()];

    for mut command in groups {
        commands.append(command.as_mut());
    }

    commands
}
