// lib.rs
use anyhow::{Result};
use chrono::{DateTime, Utc};
use humantime::parse_duration;
use notify_rust::Notification;
use sqlx::{sqlite::SqlitePool, FromRow};
use tokio::time;

pub mod config {
    use anyhow::{Result, anyhow};
    /*
    Why choose anyhow for error handling?
    anyhow provides easy error handling context, automatic error type conversion,
    and simplified error propagation (? operator)
    */

    #[derive(Debug)]
    pub struct Config {
        pub api_key: String,
    }

    impl Config {
        pub fn from_env() -> Result<Self> {
            Ok(Self {
                api_key: std::env::var("DEEPSEEK_API_KEY")
                    .map_err(|_| anyhow!("Please set DEEPSEEK_API_KEY in .env file"))?,
            })
        }
    }
}

/// Database models
mod models {
    use super::*;

    #[derive(Debug, FromRow)]
    /*
    FromRow is sqlx trait for automatic row-to-struct conversion
    derive macro generates implementation code automatically
    */
    pub struct Reminder {
        pub id: i64, // 64-bit signed integer matching SQLite INTEGER primary key
        pub message: String,
        pub trigger_time: String,
        pub ai_advice: Option<String>, // NULLable field uses Option
    }
}

/// Initialize database
pub async fn init_db() -> Result<SqlitePool> {
    db::init().await
}

/// Create reminder
pub async fn create_reminder(pool: &SqlitePool, time_str: &str, message: &str) -> Result<(i64, Option<String>)> {
    let config = config::Config::from_env()?;
    let advice = match deepseek::get_simple_advice(&config.api_key, message).await {
        Ok(a) => {
            println!("✓ AI advice generated successfully");
            Some(a)
        }
        Err(e) => {
            eprintln!("⚠️ Failed to generate advice: {}", e);
            None
        }
    };
    let trigger_time = time_utils::parse(time_str)?;
    let trigger_str = trigger_time.to_rfc3339();
    let id = sqlx::query(
        "INSERT INTO reminders (message, trigger_time, ai_advice) VALUES (?, ?, ?)"
    )
        .bind(message)
        .bind(trigger_str)
        .bind(&advice)
        .execute(pool)
        .await?
        .last_insert_rowid();
    scheduler::schedule(pool.clone(), id, message.to_string(), trigger_time).await; //为什么这里要用pool.clone。为什么await后面没有？
    Ok((id, advice))
}

/// List all reminders
pub async fn list_reminders(pool: &SqlitePool) -> Result<()> {
    let reminders = db::get_all_reminders(pool).await?;

    if reminders.is_empty() {
        println!("No pending reminders");
        return Ok(());
    }

    let now = Utc::now();
    for r in reminders {
        match DateTime::parse_from_rfc3339(&r.trigger_time) {
            Ok(trigger_fixed) => {
                let trigger_utc = trigger_fixed.with_timezone(&Utc);
                let remain = trigger_utc - now;
                println!( 
                    "#{} - {}\n  Time remaining: {}\n  Advice: {}",
                    r.id,
                    r.message,
                    humantime::format_duration(remain.to_std().unwrap_or_default()),
                    r.ai_advice.as_deref().unwrap_or("None")
                );
            }
            Err(e) => eprintln!("Failed to parse time: {}", e),
        }
    }

    Ok(())
}

/// Database operations
mod db {
    use crate::models::Reminder;
    use super::*;

    pub async fn init() -> Result<SqlitePool> {
        let pool = SqlitePool::connect("sqlite:reminders.db?mode=rwc").await?;
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS reminders (
        id INTEGER PRIMARY KEY,
        message TEXT NOT NULL,
        trigger_time TEXT NOT NULL,
        ai_advice TEXT  -- ALLOW NULL
    )"#,
        )
            .execute(&pool)
            .await?;
        Ok(pool)
    }

    pub async fn get_all_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>> {
        let reminders = sqlx::query_as::<_, Reminder>("SELECT * FROM reminders ORDER BY trigger_time")
            .fetch_all(pool)
            .await?;
        Ok(reminders)
    }

    pub async fn delete_reminder(pool: &SqlitePool, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM reminders WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

/// Scheduler implementation
mod scheduler {
    use super::*;

    pub async fn schedule(pool: SqlitePool, id: i64, message: String, trigger: DateTime<Utc>) {
        tokio::spawn(async move {
            let wait_duration = trigger.signed_duration_since(Utc::now());

            if wait_duration.num_seconds() > 0 {
                if let Ok(d) = wait_duration.to_std() {
                    time::sleep(d).await;
                }
            }

            if let Err(e) = Notification::new().body(&message).show() {
                eprintln!("Notification failed: {}", e);
            }

            if let Err(e) = db::delete_reminder(&pool, id).await {
                eprintln!("Deletion failed: {}", e);
            }
        });
    }
}

/// Time utilities
pub mod time_utils {
    use super::*;

    pub fn parse(input: &str) -> Result<DateTime<Utc>> {
        if let Some(duration_str) = input.strip_prefix('+') {
            let duration = parse_duration(duration_str)?;
            Ok(Utc::now() + chrono::Duration::from_std(duration)?)
        } else {
            let dt = DateTime::parse_from_rfc3339(input)?;
            Ok(dt.with_timezone(&Utc))
        }
    }
}

/// CLI definitions
pub mod cli {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Command,
    }

    #[derive(Subcommand)]
    pub enum Command {
        /// Create a reminder (format: +1h or 2023-12-31T23:59:00Z)
        Create {
            time: String,
            message: String,
        },
        /// List all reminders
        List,
    }
}

/// DeepSeek client
mod deepseek {
    use anyhow::Context;

    pub async fn get_simple_advice(api_key: &str, message: &str) -> anyhow::Result<String> {
        let prompt = format!("Please generate brief advice for this reminder: {}", message);
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({ // 为什么这里的json!有一个！
                "messages": [{"role": "user", "content": prompt}],
                "model": "deepseek-chat",
                "temperature": 0.5
            }))
            .send() // 这个send是什么意思？
            .await // 这个await的意思？
            .context("Failed to send request to Deepseek API")?; // 这又是使用了什么错误处理？

        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse API response as JSON")?;

        // 结构化校验（带详细错误信息），请解释json是什么
        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(||
                anyhow::anyhow!(
                    "Malformed API response: missing content field\nFull response: {}",
                    json
                )
            )
    }
}

