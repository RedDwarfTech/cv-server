# build stage
FROM rust:1.54-bullseye as builder
WORKDIR /app
COPY . /app
RUN rustup default stable
RUN cargo build --release
# RUN cargo build
# do not use slim image, will block when query database
FROM debian:bullseye
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
# ENV ROCKET_PORT=11014
RUN apt-get update && apt-get install libpq5 curl -y
COPY --from=builder /app/.env /app
COPY --from=builder /app/settings.toml /app
#
# only copy the execute file to minimal the image size
# do not copy the release folder
COPY --from=builder /app/target/release/cv-server /app/
COPY --from=builder /app/Rocket.toml /app
RUN mkdir -p /app/config/
COPY --from=builder /app/log4rs.yaml /app/config/
CMD ["./cv-server"]



