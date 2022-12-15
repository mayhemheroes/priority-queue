FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /priority-queue
WORKDIR /priority-queue/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /priority-queue/fuzz/target/x86_64-unknown-linux-gnu/release/prio-queue-fuzz /