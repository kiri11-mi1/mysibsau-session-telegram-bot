mod msg_templates;
mod pallada_service;
mod schemas;

use msg_templates::*;
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
        bot.send_message(msg.chat.id, DEFAULT_RESPONSE).await?;
        return Ok(());
    }
    match msg_text.unwrap() {
        "/start" => bot.send_message(msg.chat.id, START).await?,
        "/help" => bot.send_message(msg.chat.id, HELP).await?,
        _ => {
            let exams = search_exams(msg_text.unwrap()).await;
            bot.send_message(msg.chat.id, exams).await?
        }
    };
    Ok(())
}

async fn search_exams(group: &str) -> String {
    let pds = PalladaService::new().await;
    if pds.is_none() {
        return SERVER_ERROR.to_string();
    }
    let mut pds_unwrapped = pds.unwrap();
    let group_id = pds_unwrapped.find_group_by_name(String::from(group)).await;
    if group_id.is_none() {
        return GROUP_NOT_FOUND.to_string();
    }
    let timetable_list = pds_unwrapped.get_exams_timetable(group_id.unwrap()).await;
    if timetable_list.is_none() {
        return EXAMS_DO_NOT_EXIS.to_string();
    }
    let exams = timetable_list.unwrap();
    for exam in exams {
        println!("{}", exam);
    }

    let result_text = format!("Группа {}.", group);
    return result_text;
}
