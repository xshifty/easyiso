FROM node:20-bullseye-slim AS node

FROM rust:1.80-slim-bullseye AS build_app
WORKDIR /usr/src/app
COPY . .
COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/share /usr/local/share
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin
RUN npm install
RUN cargo build --release
RUN cargo test --release

FROM debian:bullseye-slim AS runtime
WORKDIR /usr/src/app
COPY --from=build_app /usr/src/app/static ./static
COPY --from=build_app /usr/src/app/templates ./templates
COPY --from=build_app /usr/src/app/target/release/easyiso .
CMD ["./easyiso"]

EXPOSE 8000
