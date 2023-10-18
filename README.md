# mysibsau-session-telegram-bot
Бот для отображения расписания экзменов в телеграме


# Что нужно сделать чтобы запустить:
1. docker/docker-compose
2. `.env` в корне проекта по примеру `.env.example`
3. make утилита, чтобы запускать `Makefile` 

# Как рулить проектом
1. `make launch` - Сборка и запуск проекта в докере
2. `make reload` - Удаление контейнеров + сборка и запуск проекта в докере
3. `make down` - Удаление контейнеров
4. `make restart` - Удаление контейнеров + запуск проекта в докере
5. `make up` - Запуск проекта в докере
6. `make build` - Сборка проекта в докере
