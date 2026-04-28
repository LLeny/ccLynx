FROM debian:latest
LABEL Name=ccLynx Version=0.0.1
RUN apt-get -y update && apt-get install -y git make gcc imagemagick bc xxd sox curl fzf ripgrep tree git gettext libtool libtool-bin autoconf automake cmake g++ pkg-config zip unzip

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y

WORKDIR /opt
RUN git clone https://codeberg.org/42Bastian/lyxass.git --depth=1 && cd lyxass && make && cp lyxass /usr/local/bin

RUN PATH="/usr/local/bin:$PATH"

WORKDIR /workspace
