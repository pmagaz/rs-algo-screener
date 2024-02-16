# GLOBAL VARS
ARG APP_NAME=rs_algo_client
ARG TARGET=wasm32-unknown-unknown
# BUILDER
FROM rust:1-buster as builder
ARG APP_NAME
ARG TARGET

WORKDIR ./$APP_NAME
#RUN trunk build 
COPY dist ./
ADD . ./


#IMAGE 
FROM nginx:stable-alpine
ARG APP_NAME
ARG APP_DIR=/usr/share/nginx/www

COPY --from=builder $APP_NAME/dist $APP_DIR 
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY .htpasswd /etc/nginx/.htpasswd

EXPOSE 8000
CMD ["nginx", "-g", "daemon off;"]
