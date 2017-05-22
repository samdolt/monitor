FROM debian:jessie
MAINTAINER Jimmy Cuadra <jimmy@jimmycuadra.com>

ENV USER root
ENV RUST_VERSION=1.17.0

RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    libssl-dev \
    pkg-config && \
  curl -sO https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
  DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
  DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
  rm -rf \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/* && \
  mkdir /source && \
  DEBIAN_FRONTEND=noninteractive apt-get update -yqq && \
  DEBIAN_FRONTEND=noninteractive apt-get install -yqq --no-install-recommends build-essential && \
  DEBIAN_FRONTEND=noninteractive apt-get install -yqq rubygems python-pygments && \
  gem install asciidoctor && \
  gem install asciidoctor-pdf --pre && \
  gem install pygments.rb

VOLUME ["/source"]
WORKDIR /source
CMD ["/bin/bash"]
