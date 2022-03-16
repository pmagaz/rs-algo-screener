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
ENV TZ=CET \
  APP_USER=dev 

#DEPENDENCIES
RUN apk update \
  && apk add fontconfig \
  && apk add su-exec sudo\
  && apk add ttf-opensans \
  && rm -rf /var/cache/apk/*

#USER
RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER \
  && echo '%wheel ALL=(ALL) ALL' >> \
  /etc/sudoers.d/wheel \
  && adduser $APP_USER wheel \
  && chown -R $APP_USER:$APP_USER ${APP_DIR}

#FONT
ENV FONTCONFIG_PATH=/etc/fonts
# COPY ./DejaVuSans.ttf ./
# RUN mkdir -p /usr/share/fonts/truetype/
# RUN install -m644 DejaVuSans.ttf /usr/share/fonts/truetype/
COPY ./fonts.conf ./
RUN sudo mv fonts.conf /etc/fonts/

COPY --from=builder /home/rust/src/$APP_NAME/target/$TARGET/release/$APP_NAME ${APP_DIR}/$APP_NAME

USER $APP_USER
WORKDIR ${APP_DIR}
CMD ["./rs_algo_backend"]