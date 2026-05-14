use anyhow::{Result, bail};
use poise::serenity_prelude::UserId;
use serde::Deserialize;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{path::PathBuf, str::FromStr};

use crate::application::infra::env;

const APPSETTINGS_PATH: &str = "appsettings_{ENV}.json";

fn try_get_file(max_depth: usize, filename: String) -> Result<PathBuf> {
    for i in 0..max_depth {
        let try_path = format!("{}{}", "../".repeat(i), &filename);
        let possible_path = std::path::Path::new(&try_path);

        match possible_path.exists() {
            true => return Ok(possible_path.to_path_buf()),
            false => continue,
        }
    }

    bail!("The file {} was not found in depth {}", filename, max_depth)
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub allowed_ids: Vec<UserId>,
    pub prefix: String,
}

impl AppSettings {
    pub fn load() -> Result<Self> {
        env::init()?;
        let env = env::get("ENV").unwrap_or_else(|_| "dev".to_owned());

        let filename = APPSETTINGS_PATH.replace("{ENV}", env.as_str());

        let path = try_get_file(5, filename)?;

        let settings_json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&settings_json)?)
    }
}

pub async fn create_sqlite_pool() -> Result<Pool<Sqlite>> {
    let connection_string =
        env::get("SQLITE_DATABASE_URL").unwrap_or_else(|_| "sqlite://acn_r.db".to_owned());

    let options = SqliteConnectOptions::from_str(&connection_string)?.create_if_missing(true);

    Ok(SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?)
}

pub async fn initialize_database(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(pool)
        .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS guilds (id INTEGER PRIMARY KEY NOT NULL)")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS guild_name_changes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            name TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_guild_name_changes_lookup
            ON guild_name_changes (guild_id, date DESC)",
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL)")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS signatures (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            emojis TEXT NOT NULL,
            date TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_signatures_lookup
            ON signatures (user_id, date DESC)",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS nickname_changes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            guild_id INTEGER,
            nickname TEXT NOT NULL,
            date TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_nickname_changes_lookup
            ON nickname_changes (user_id, date DESC)",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS command_uses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id INTEGER,
            user_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            name TEXT NOT NULL,
            args TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS command_errors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id INTEGER,
            user_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            name TEXT NOT NULL,
            args TEXT NOT NULL,
            error TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS russian_roulette (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            shot INTEGER NOT NULL,
            number_drawn REAL NOT NULL,
            date TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            guild_id INTEGER,
            command TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS jukebox_uses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            time TEXT NOT NULL,
            author TEXT NOT NULL,
            title TEXT NOT NULL,
            uri TEXT,
            seconds INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
