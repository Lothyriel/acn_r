use anyhow::Result;
use chrono::{DateTime, Utc};
use poise::serenity_prelude::Mentionable;
use rand::Rng;

use crate::{
    application::models::{
        dto::command_use::CommandUseDto, entities::russian_roulette::RussianRoulette,
    },
    extensions::{
        log_ext::LogExt,
        serenity::{context_ext::ContextExt, Context},
    },
};

async fn after(ctx: Context<'_>) -> Result<()> {
    let now = chrono::Utc::now();

    let guild_info = ctx.get_guild_info();

    let nickname = ctx.get_author_name().await;

    attempt_russian_roulette(ctx, now).await?;

    let command_name = ctx.command().name.to_owned();

    let dto = CommandUseDto {
        date: now,
        guild_info,
        user_id: ctx.author().id.get(),
        user_nickname: nickname,
        command: command_name,
        args: ctx.get_command_args().await,
    };

    ctx.data().repositories.command.add_command_use(dto).await?;

    Ok(())
}

async fn attempt_russian_roulette(ctx: Context<'_>, now: DateTime<Utc>) -> Result<()> {
    let random_number: f32 = rand::thread_rng().gen();

    let shot = random_number < 0.01;

    if shot {
        let message = format!(
            "Comi o cu do {} (Tirou {:.4})",
            ctx.author().mention(),
            random_number
        );

        ctx.say(message).await?;
    }

    let stats_repository = &ctx.data().repositories.stats;

    let attempt = RussianRoulette {
        shot,
        guild_id: ctx.guild_id().map(|g| g.get()),
        user_id: ctx.author().id.get(),
        date: now,
        command: ctx.command().name.to_owned(),
        number_drawn: random_number,
    };

    stats_repository.add_russian_roulette(attempt).await?;

    Ok(())
}

pub async fn handler(ctx: Context<'_>) {
    after(ctx).await.log();
}
