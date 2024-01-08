FROM clux/muslrust:1.75.0 as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM alpine
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/hcal .
ENTRYPOINT [ "/hcal" ]

