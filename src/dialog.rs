/*
sketch of dialogue
- eg: l15 n4 sc- 
<n4>
- here is your 4 generations:
- <generation 1>
- <generation 2>
- <generation 3>
- <generation 4>
*/
use std::str::FromStr;
use crate::comms::GenPref;
use crate::rng::generate_string;


use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*, 
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId},
    utils::{command::BotCommands, html, markdown}
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    ReceiveInfo,
    ReceiveGenChoise{
        prms: String, // parameters for generation
        msg_id: i32
    },   
}

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "to see this message")]
    Help,
    #[command(description = "cancel dialogue.")]
    Cancel,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveInfo].endpoint(receive_info))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
    .branch(
        case![State::ReceiveGenChoise { prms, msg_id }].endpoint(receive_generation_selection),
    );


    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}


async fn help(bot: Bot, msg: Message) -> HandlerResult {
    let name = msg.chat.first_name().unwrap_or("my fav user");
    let about_commands = GenPref::about_me();
    bot.send_message(msg.chat.id,
        format!("Hey {name}.\n\n{about_commands}\n\n/cancel : to cancel current parrameters."))
        .parse_mode(teloxide::types::ParseMode::Html)
        .await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue. Type your new parameters.").await?;
    dialogue.exit().await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Select one of gens above. Type /help to see the usage of bot.")
        .await?;
    Ok(())
}

async fn send_invalid_message(bot: Bot, msg: Message) -> HandlerResult {
    let simple_example_html = html::code_inline(GenPref::simple_example());

    bot.send_message(msg.chat.id, format!("Please, send me valid preference for gen(s).\
        \nE.g. {simple_example_html}\
        \nOr send /help command to discover which options awaible."))
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await?;
    Ok(())
}


fn keyboard(gens: Vec<String>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for gen in gens{
        keyboard.push(vec![InlineKeyboardButton::callback(&gen, &gen)]);
    }

    InlineKeyboardMarkup::new(keyboard)
}


async fn receive_info(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(prms) => {
            if let Ok(prf) = GenPref::from_str(&prms) {
                let gens = generate_string(prf);
                let tmp_msg = bot.send_message(msg.chat.id, "Select one of gens:")
                .reply_markup(keyboard(gens))
                .await?;

                dialogue.update(State::ReceiveGenChoise { prms, msg_id: tmp_msg.id.0 }).await?;    
            } else {
                send_invalid_message(bot, msg).await?;
            }
        }

        None => {
            send_invalid_message(bot, msg).await?;
        }
    }

    Ok(())
}

async fn receive_generation_selection(
    bot: Bot,
    dialogue: MyDialogue,
    (_prms, msg_id): (String, i32), // Available from `State::ReceiveGenChoice`.
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(gen) = &q.data {
        bot.delete_message(dialogue.chat_id(), MessageId(msg_id)).await?;
        bot.send_message(
            dialogue.chat_id(),
             markdown::code_inline(gen),
        )
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
        dialogue.exit().await?;
    }

    Ok(())
}