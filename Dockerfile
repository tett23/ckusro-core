FROM rust:1.32-slim

ENV LANG C.UTF-8

WORKDIR /app

RUN apt-get update -y -q
RUN apt-get install -y curl gnupg

# install dependency package
RUN apt-get update -y -q
RUN apt-get install -y apt-transport-https libssl-dev

# install nodejs
RUN curl -s -L git.io/nodebrew | perl - setup
ENV PATH /root/.nodebrew/current/bin:$PATH
RUN nodebrew install-binary v10.15.0
RUN nodebrew use v10.15.0

# install yarn
# https://yarnpkg.com/en/docs/install#linux-tab
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update
RUN apt-get install -y yarn

RUN yarn install