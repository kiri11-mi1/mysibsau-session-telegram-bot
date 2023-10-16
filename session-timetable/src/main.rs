mod pallada_service;

use pallada_service::PalladaService;
use teloxide::prelude::*;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message) -> ResponseResult<()> {
    let msg_text = msg.text();
    if msg_text.is_none() {
        bot.send_message(msg.chat.id, "Не понимаю. Введите текстовое сообщение.")
            .await?;
        return Ok(());
    }
    match msg_text.unwrap() {
        "/start" => bot.send_message(msg.chat.id, "Привет, чтобы узнать расписание сессии, просто введи название своей группы. Регистр не важен.").await?,
        _ => {
            let exams = search_exams(msg_text.unwrap()).await;
            bot.send_message(msg.chat.id, exams).await?   
        }
    };
    Ok(())
}

async fn search_exams(group: &str) -> String {
    let mut pds = PalladaService::new().await;
    let group_id = pds.find_group_by_name(String::from(group)).await;
    if group_id.is_none() {
        return String::from("Группа с таким именем не найдена.");
    }
    let result_text = format!("Группа {} имеет id {}", group, group_id.unwrap());
    return result_text
}
