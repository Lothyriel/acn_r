use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::prelude::{Message, UserId},
    prelude::Context,
};
use std::collections::HashSet;

#[help]
#[individual_command_tip = "Comando de ajuda vei, se quer de um comando especifico use !help <commando>"]
#[command_not_found_text = "Não existe: `{}` teu"]
#[max_levenshtein_distance(3)]
#[indention_prefix = "-"]
#[lacking_permissions = "Strike"]
#[lacking_role = "Strike"]
#[wrong_channel = "Strike"]
pub async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await?;
    Ok(())
}
