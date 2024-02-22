FROM rust:alpine as build
WORKDIR /build

RUN apk add musl-dev

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src

RUN cargo build --release

FROM scratch
WORKDIR /app

ENV PATH="$PATH:/app/bin"

COPY --from=build /build/target/release/philipshue-cli /app/bin/philipshue-cli

ENV PHILIPSHUE_CONFIG=/config/config.toml
VOLUME [ "/config" ]

CMD ["/app/bin/philipshue-cli"]