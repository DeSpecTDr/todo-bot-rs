mod database;

use std::str::FromStr;

use anyhow::{bail, Context, Result};
use chrono::{prelude::*, Days};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::SqlitePool;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting database...");
    let db = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

    tracing::info!("Starting todo bot...");
    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(command_handler))
        .branch(dptree::endpoint(invalid_command));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display help.")]
    Help,
    #[command(description = "add a task. /add <date> <task>", parse_with = "split")]
    Add { time: Time, description: String },
    #[command(description = "complete a task. /done <id>")]
    Done(u32),
    #[command(description = "list tasks.")]
    List,
}

async fn command_handler(db: SqlitePool, bot: Bot, msg: Message, cmd: Command) -> Result<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Add { time, description } => {
            database::add_todo(&db, msg.chat.id.0, time.0, description).await?;
            bot.send_message(msg.chat.id, "Task is added.").await?;
        }
        Command::Done(task_id) => {
            if database::remove_todo(&db, msg.chat.id.0, task_id).await? {
                bot.send_message(msg.chat.id, "Task is removed.").await?;
            }
        }
        Command::List => {
            let tasks = database::list_todos(&db, msg.chat.id.0).await?;
            let formatter = tasks
                .iter()
                .enumerate()
                .format_with("\n", |elt, f| f(&format_args!("{}: {}", elt.0 + 1, elt.1)));
            bot.send_message(msg.chat.id, format!("Your tasks:\n{}", formatter)).await?;
        }
    }

    Ok(())
}

#[derive(Clone)]
struct Time(NaiveDate);

impl FromStr for Time {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        // let (days, _) = s.split_once(|c| !c.is_ascii_alphanumeric()).ok_or("")?;
        // let days = days.parse::<u32>();
        // match s {
        //     "d" => Ok(Self::Days(0)),
        //     _ => bail!("Allowed units: h, m, s"),
        // }
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\+(.*)d$").unwrap());

        if let Ok(date) = NaiveDate::from_str(s) {
            return Ok(Self(date));
        }

        if let Some(days) = REGEX.captures(s).and_then(|v| v.get(1)) {
            let days = days.as_str().parse::<u64>()?;
            return Ok(Self(
                Local::now()
                    .date_naive()
                    .checked_add_days(Days::new(days))
                    .context("Day overflow")?,
            ));
        }

        bail!("Can't parse");
        // Local::now().date_naive();
        // Ok(Self(NaiveDate::from_ymd_opt(2022, 12, 25).unwrap()))
    }
}

async fn invalid_command(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}
