# syntax=docker.io/docker/dockerfile:1.7-labs
FROM node:20-bullseye-slim AS node

FROM rust:1.80-slim-bullseye AS build_app
WORKDIR /usr/src/app
COPY --exclude=docker . .
COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/share /usr/local/share
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
RUN npm install
RUN cargo install cargo-binstall
RUN cargo binstall cargo-watch -y
EXPOSE 8000
