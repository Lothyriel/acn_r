use poise::{command, serenity_prelude::UserId};

use crate::extensions::serenity::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "R34")]
pub async fn get_voice(
    ctx: Context<'_>,
    #[description = "User to get voice"] user: Option<UserId>,
) -> CommandResult {
    
    Ok(())
}
