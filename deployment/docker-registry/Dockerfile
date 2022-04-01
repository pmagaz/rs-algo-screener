#IMAGE
FROM alpine:latest

RUN apk update \
  && apk add docker-registry

ADD ./config.yml /etc/docker-registry/config.yml

EXPOSE 6000
CMD ["/usr/bin/docker-registry", "serve", "/etc/docker-registry/config.yml"]