# GLOBAL VARS
ARG APP_NAME=rs_algo_backend
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

#ENV
ARG APP_DIR=/usr/src/$APP_NAME
EXPOSE 8000
ENV APP_USER=dev 

#DEPENDENCIES
RUN apk update \
  && apk add --no-cache tzdata \
  && cp /usr/share/zoneinfo/Europe/Madrid /etc/localtime && echo Europe/Madrid > /etc/timezone \
  && apk add --no-cache fontconfig \
  && apk add --no-cache ttf-opensans \
  && apk add --no-cache su-exec sudo \
  && rm -rf /var/cache/apk/*

#USER
RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER \
  && echo '%wheel ALL=(ALL) ALL' >> \
  /etc/sudoers.d/wheel \
  && adduser $APP_USER wheel

#FONT MESS
COPY ./fonts.conf ./
RUN sudo mv fonts.conf /etc/fonts/
RUN sudo rm -Rf /etc/fonts/conf.d/*.conf
RUN sudo find /usr/share/fonts/TTF/*.ttf | grep -v OpenSans-Regular.ttf | xargs rm -r
RUN fc-cache -fv

COPY --from=builder /home/rust/src/$APP_NAME/target/$TARGET/release/$APP_NAME ${APP_DIR}/$APP_NAME

RUN chown -R $APP_USER:$APP_USER ${APP_DIR}

USER $APP_USER
WORKDIR ${APP_DIR}
CMD ["./rs_algo_backend"]