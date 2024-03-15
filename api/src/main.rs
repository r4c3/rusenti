use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

use skrudriver::config::Config;
use skrudriver::api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Config::parse();

    println!();
    
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://dev:notprod@localhost:5432/skrudriver")
        .await
        .context("could not connect to database")?;

    // sqlx::migrate!().run(&db).await?;

    api::serve(config, db).await?;

    Ok(())
}
