
mod comms;
mod rng;
mod dialog;

use dialog::{schema, State};

use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
};

#[tokio::main]
async fn main() {

    pretty_env_logger::init();
    log::info!("Starting tg bot...");

    let bot = Bot::from_env();
    
    Dispatcher::builder(bot, schema())
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}