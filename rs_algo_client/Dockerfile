# GLOBAL VARS
ARG APP_NAME=rs_algo_client
ARG TARGET=wasm32-unknown-unknown
# BUILDER
FROM rust:1-buster as dist
ARG APP_NAME
ARG TARGET

#RUN rustup toolchain install nightly
#RUN rustup target add $TARGET
#RUN USER=root cargo install trunk --locked
WORKDIR ./$APP_NAME
#RUN trunk build 
COPY dist ./
ADD . ./


#IMAGE 
FROM nginx:stable-alpine
ARG APP_NAME
ARG APP_DIR=/usr/share/nginx/www

COPY --from=dist $APP_NAME/dist $APP_DIR 
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 8000
CMD ["nginx", "-g", "daemon off;"]
