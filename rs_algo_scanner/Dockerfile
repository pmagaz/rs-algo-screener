# GLOBAL VARS
ARG APP_NAME=rs_algo_scanner
ARG TARGET=aarch64-unknown-linux-musl
# BUILDER
FROM messense/rust-musl-cross:aarch64-musl as builder
ARG APP_NAME
ARG TARGET

RUN rustup target add $TARGET

RUN USER=root cargo new --bin $APP_NAME
WORKDIR ./$APP_NAME
COPY Cargo.toml ./
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN rm ./target/$TARGET/release/deps/$APP_NAME*
RUN cargo build --release 

#IMAGE 
FROM alpine:latest
ARG APP_NAME
ARG TARGET

ARG APP_DIR=/usr/src/$APP_NAME
ENV APP_USER=dev

#DEPENDENCIES
RUN apk update \
  && apk add --no-cache tzdata curl \
  && cp /usr/share/zoneinfo/Europe/Madrid /etc/localtime && echo Europe/Madrid > /etc/timezone \
  && rm -rf /var/cache/apk/*

#USER
RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER 

COPY --from=builder /home/rust/src/$APP_NAME/target/$TARGET/release/$APP_NAME ${APP_DIR}/$APP_NAME

RUN chown -R $APP_USER:$APP_USER ${APP_DIR}

USER $APP_USER
WORKDIR ${APP_DIR}
CMD ["./rs_algo_scanner"]