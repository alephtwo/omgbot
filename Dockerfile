FROM ruby:2.5-alpine

# Set a working directory and copy files in
WORKDIR /app
COPY . /app

ENV BUILD_DEPS curl-dev ruby-dev build-base
ENV APP_DEPS libsodium libsodium-dev opus opus-dev ffmpeg
ENV RUBY_DEPS ruby-io-console ruby-bundler

# Install Dependencies
RUN apk update \
  && apk upgrade \
  && apk add ${BUILD_DEPS} \
  && apk add ${APP_DEPS} \
  && apk add ${RUBY_DEPS} \
  && bundle install

ENTRYPOINT [ "ruby", "omg.rb" ]
