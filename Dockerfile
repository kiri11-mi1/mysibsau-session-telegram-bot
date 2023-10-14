FROM rust:1.73.0

ADD ./session-timetable /app

WORKDIR /app

RUN cargo build --release


CMD ["./target/release/session-timetable"]