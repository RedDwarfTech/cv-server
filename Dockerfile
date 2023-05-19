# build stage
FROM rust:1.69.0-alpine3.17 as builder
WORKDIR /app
COPY . /app
RUN rustup default stable
RUN cargo build --release
# RUN cargo build
FROM rust:1.69.0-alpine3.17
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
# ENV ROCKET_PORT=11014
RUN apk add musl-dev -y
COPY --from=builder /app/.env /app
COPY --from=builder /app/settings.toml /app
#
# only copy the execute file to minimal the image size
# do not copy the release folder
COPY --from=builder /app/target/release/cv-server /app/
COPY --from=builder /app/Rocket.toml /app
CMD ["./cv-server"]



