// main.rs
use anyhow::Result;
use clap::Parser;
use reminder_cli::{cli, init_db, list_reminders, create_reminder};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    /*
    Environment variables are key-value pairs provided by the operating system
    dotenv().ok() reads the .env file in the project root and loads defined environment variables
    (like DEEPSEEK_API_KEY=xxx) into the process environment
    */
    let pool = init_db().await?;
    /*
    init_db() is an async function returning a Future
    .await suspends the current task until the Future completes, obtaining the database connection pool
    ? propagates errors: if initialization fails, return error immediately
    */
    let args = cli::Cli::parse();
    /*
    Using clap library to parse command line arguments
    Cli struct defines the command line interface
    parse() parses user input (like create or list commands), returns structured data
    */

    match args.command {
        /*
        match is Rust's pattern matching for handling enum variants
        Executes different logic based on user commands (Create or List)
        */
        cli::Command::Create { time, message } => {
            let (id, advice) = create_reminder(&pool, &time, &message).await?;
            println!("✓ Reminder #{} created", id);
            if let Some(content) = advice {
                /*
                Option type represents either Some(value) or None
                if let safely unwraps possible AI advice content
                */
                println!("══════════ AI Advice ══════════");
                println!("{}", content);
                println!("═══════════════════════════════");
            } else {
                println!("⚠️ No AI advice generated");
            }
        }

        cli::Command::List => {
            list_reminders(&pool).await?;
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(())
}