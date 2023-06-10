FROM rust as compile
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/base-debian11
WORKDIR /
COPY --from=compile /app/target/release/aegis /aegis
ENV PORT=80
ENTRYPOINT ["/aegis"]