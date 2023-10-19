FROM rust:1.73.0-buster as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin session-timetable
WORKDIR /session-timetable

# 2. Copy our manifests
COPY ./session-timetable/Cargo.lock ./Cargo.lock
COPY ./session-timetable/Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./session-timetable/src ./src

# 5. Build for release.
RUN rm ./target/release/deps/session_timetable*
RUN cargo build --release

# our final base
FROM rust:1.73.0-slim-buster

# copy the build artifact from the build stage
COPY --from=build /session-timetable/target/release/session-timetable .

CMD ["./session-timetable"]
