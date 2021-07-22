FROM rust:1.30

WORKDIR /opt/particles
COPY . .
RUN [[ -f Cargo.lock ]] && rm Cargo.lock || true
RUN cargo build
CMD ["particles"]
