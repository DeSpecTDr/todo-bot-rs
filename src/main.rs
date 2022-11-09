mod database;

use futures::executor::block_on;
use once_cell::sync::Lazy;
use sqlx::SqlitePool;
use teloxide::{prelude::*, utils::command::BotCommands};

static DB: Lazy<SqlitePool> =
    Lazy::new(|| block_on(SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap())).unwrap());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting todo bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
    // let handler = Update::filter_message()
    //     .enter_dialogue::<Message, ErasedStorage<State>, State>()
    //     .branch(dptree::case![State::Start].endpoint(start))
    //     .branch(
    //         dptree::case![State::GotNumber(n)]
    //             
    // .branch(dptree::entry().filter_command::<Command>().endpoint(got_number))
    //             .branch(dptree::endpoint(invalid_command)),
    //     );

    // Dispatcher::builder(bot, handler)
    //     .dependencies(dptree::deps![storage])
    //     .enable_ctrlc_handler()
    //     .build()
    //     .dispatch()
    //     .await;

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Start,
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
    #[command(description = "add a task.")]
    Add(String),
    #[command(description = "complete a task.")]
    Done(i64),
    #[command(description = "list tasks.")]
    List,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start | Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Add(description) => {
            database::add_todo(&DB, msg.chat.id.0, description).await.unwrap();
            bot.send_message(msg.chat.id, "Task is added.").await?;
        }
        Command::Done(task_id) => {
            database::remove_todo(&DB, msg.chat.id.0, task_id).await.unwrap();
            bot.send_message(msg.chat.id, "Task is removed.").await?;
        },
        Command::List => {
            bot.send_message(msg.chat.id, database::list_todos(&DB, msg.chat.id.0).await.unwrap())
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Command is not found.").await?;
        }
    };

    Ok(())
}

// type MyDialogue = Dialogue<State, ErasedStorage<State>>;
// type MyStorage = std::sync::Arc<ErasedStorage<State>>;

// #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
// pub enum State {
//     #[default]
//     Start,
//     GotNumber(i32),
// }

// async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) ->
// anyhow::Result<()> {     match msg.text().map(|text| text.parse::<i32>()) {
//         Some(Ok(n)) => {
//             dialogue.update(State::GotNumber(n)).await?;
//             bot.send_message(
//                 msg.chat.id,
//                 format!("Remembered number {n}. Now use /get or /reset."),
//             )
//             .await?;
//         }
//         _ => {
//             bot.send_message(msg.chat.id, "Please, send me a
// number.").await?;         }
//     }

//     Ok(())
// }

// async fn got_number(
//     bot: Bot,
//     dialogue: MyDialogue,
//     num: i32, // Available from `State::GotNumber`.
//     msg: Message,
//     cmd: Command,
// ) -> anyhow::Result<()> {
//     match cmd {
//         Command::Get => {
//             bot.send_message(msg.chat.id, format!("Here is your number:
// {num}.")).await?;         }
//         Command::Reset => {
//             dialogue.reset().await?;
//             bot.send_message(msg.chat.id, "Number reset.").await?;
//         }
//         _ => {}
//     }
//     Ok(())
// }

async fn invalid_command(bot: Bot, msg: Message) -> anyhow::Result<()> {
    // bot.send_message(msg.chat.id, "Please, send /get or /reset.").await?;
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}
