import logging

from aiogram import Bot, Dispatcher, executor, types

from constants import (
    TELEGRAM_TOKEN, START_CMD, HELP_CMD, EXAM_INFO,
    NOT_EXAMS, EXAMS_ARE_EXIST, EXAM_NUMBER, SEVER_RESPONSE_MESSAGE
)
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
    result_message = ''
    exams, error = get_session_timetable(message.text)

    if error:
        result_message += NOT_EXAMS.format(group_name=message.text)
        result_message += SEVER_RESPONSE_MESSAGE.format(server_message=error)
        return await bot.send_message(message.chat.id, result_message, parse_mode='HTML')

    result_message += EXAMS_ARE_EXIST.format(group_name=message.text)
    for i, exam in enumerate(exams):
        result_message += EXAM_NUMBER.format(number=i+1)
        result_message += EXAM_INFO.format(**exam)
    return await bot.send_message(message.chat.id, result_message, parse_mode='HTML')


if __name__ == '__main__':
    executor.start_polling(dp, skip_updates=True)
