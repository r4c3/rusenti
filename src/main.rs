use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

use skrudriver::config::Config;
use skrudriver::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database")?;

    // sqlx::migrate!().run(&db).await?;

    http::serve(config, db).await?;

    Ok(())
}

