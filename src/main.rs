// This bot throws a dice on each incoming message.

mod database;

use teloxide::{dispatching::dialogue::ErasedStorage, prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Connecting to database...");
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    tracing::info!("Starting todo bot...");

    let bot = Bot::from_env();

    // Command::repl(bot, answer).await;
    let handler = Update::filter_message()
        .enter_dialogue::<Message, ErasedStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(
            dptree::case![State::GotNumber(n)]
                .branch(dptree::entry().filter_command::<Command>().endpoint(got_number))
                .branch(dptree::endpoint(invalid_command)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    // #[command(description = "handle a username.")]
    // Username(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
    // #[command(description = "get your number.")]
    // Get,
    // #[command(description = "reset your number.")]
    // Reset,
    Add,
    Done,
    List,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}

type MyDialogue = Dialogue<State, ErasedStorage<State>>;
type MyStorage = std::sync::Arc<ErasedStorage<State>>;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    GotNumber(i32),
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> anyhow::Result<()> {
    match msg.text().map(|text| text.parse::<i32>()) {
        Some(Ok(n)) => {
            dialogue.update(State::GotNumber(n)).await?;
            bot.send_message(
                msg.chat.id,
                format!("Remembered number {n}. Now use /get or /reset."),
            )
            .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Please, send me a number.").await?;
        }
    }

    Ok(())
}

async fn got_number(
    bot: Bot,
    dialogue: MyDialogue,
    num: i32, // Available from `State::GotNumber`.
    msg: Message,
    cmd: Command,
) -> anyhow::Result<()> {
    match cmd {
        Command::Get => {
            bot.send_message(msg.chat.id, format!("Here is your number: {num}.")).await?;
        }
        Command::Reset => {
            dialogue.reset().await?;
            bot.send_message(msg.chat.id, "Number reset.").await?;
        }
        _ => {}
    }
    Ok(())
}

async fn invalid_command(bot: Bot, msg: Message) -> anyhow::Result<()> {
    // bot.send_message(msg.chat.id, "Please, send /get or /reset.").await?;
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}
