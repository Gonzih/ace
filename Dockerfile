FROM rust:bullseye as base
ARG ACE_ENV=production
ENV ACE_ENV=$ACE_ENV
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get update && apt-get install -y \
    build-essential openssl nodejs libasound2-dev libudev-dev
RUN npm install -g typescript \
    && tsc --version
RUN rustup default nightly

FROM base as test
WORKDIR /ace
ADD . /ace
RUN make test

FROM base as build
WORKDIR /ace
ADD . /ace
RUN make release

FROM base as build_wasm
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli wasm-pack
WORKDIR /ace
ADD . /ace
RUN make wasm-it

FROM base as backend
RUN mkdir -p /ace
WORKDIR /ace
COPY --from=build /ace/target/release/ace_* /ace/
CMD /ace/ace_backend

FROM caddy as game_client
COPY --from=build_wasm /ace/wasm/target/* /usr/share/caddy/

FROM backend as game_server
CMD /ace/ace_game_server
