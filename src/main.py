import logging

from aiogram import Bot, Dispatcher, executor, types

from constants import TELEGRAM_TOKEN, START_CMD, HELP_CMD, EXAM_INFO, NOT_EXAMS, EXAMS_ARE_EXIST, SPLIT_LINE
from service import get_session_timetable


logging.basicConfig(
    level=logging.INFO,
    format="[%(levelname)s] -  %(name)s - (%(filename)s).%(funcName)s(%(lineno)d) - %(message)s"
)


bot = Bot(token=TELEGRAM_TOKEN)
dp = Dispatcher(bot)


@dp.message_handler(commands=['start'])
async def start(message: types.Message):
    await bot.send_message(message.chat.id, START_CMD)


@dp.message_handler(commands=['help'])
async def help(message: types.Message):
    await bot.send_message(message.chat.id, HELP_CMD)


@dp.message_handler()
async def session(message: types.Message):
    exams = get_session_timetable(message.text)
    if exams is None:
        return await bot.send_message(message.chat.id, NOT_EXAMS.format(group_name=message.text))
    result_message = EXAMS_ARE_EXIST.format(group_name=message.text)
    for exam in exams:
        result_message += EXAM_INFO.format(
            subject_name=exam['subject'][1],
            professor=exam['professor_name'],
            date=exam['date'],
            time=exam['time'],
            room=exam['room'],
            day_week=exam['day_week']
        )
        result_message += SPLIT_LINE
    return await bot.send_message(message.chat.id, result_message)


if __name__ == '__main__':
    executor.start_polling(dp, skip_updates=True)
