mod helpers;
mod msg_templates;
mod pallada_service;
mod schemas;

use msg_templates::*;
use pallada_service::PalladaService;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    log::info!("Starting exams timetable bot");

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
            bot.parse_mode(ParseMode::Html)
                .send_message(msg.chat.id, exams)
                .await?
        }
    };
    Ok(())
}

async fn search_exams(group: &str) -> String {
    let pds = PalladaService::new().await;
    if pds.is_none() {
        return SERVER_ERROR.to_string();
    }
    let mut pds_ok = pds.unwrap();
    let group_id = pds_ok.find_group_by_name(String::from(group)).await;
    if group_id.is_none() {
        return GROUP_NOT_FOUND.to_string();
    }
    let exams = pds_ok.get_exams_timetable(group_id.unwrap()).await;
    if exams.is_none() {
        return EXAMS_DO_NOT_EXIS.to_string();
    }
    let exams_ok = exams.unwrap();
    let mut result_msg = format!("🤔 Экзаменов впереди: {}.\n\n", exams_ok.len());
    for exam in exams_ok {
        result_msg += &format!(
            "📗 <b>Название дисциплины</b>: {}\n \
        👨‍🏫 <b>Преподаватель</b>: {}\n \
        🗓 <b>Дата проведения</b>: {}\n \
        ⏳ <b>Время проведения</b>: {}\n \
        🚪 <b>Кабинет</b>: {}\n \
        ⭐ <b>День недели</b>: {}\n\n",
            exam.lesson, exam.proffessor, exam.date, exam.time, exam.room, exam.day_week,
        );
    }
    return result_msg;
}
