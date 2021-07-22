# build strategy sourced from:
# - https://www.fpcomplete.com/blog/2018/07/deploying-rust-with-docker-and-kubernetes
# - http://whitfin.io/speeding-up-rust-docker-builds/

# This image will build all dependencies before you
# introducing the project's source code, which means
# they'll be cached most of the time.

FROM rust:latest as intermediate
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

ENV PATH $PATH:/root/.cargo/bin
ENV PKG_CONFIG_ALLOW_CROSS=1

# fetch dependencies using a minimal project,
# enabling the docker image to be cached with dependencies installed
RUN USER=root cargo new --bin project
WORKDIR /project

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

#RUN cargo -v build --target x86_64-unknown-linux-musl

# build actual project
#RUN rm src/*.rs
COPY ./src ./src
RUN cargo +nightly -v check
RUN cargo +nightly -v build --target x86_64-unknown-linux-musl
RUN ls -R /project/target

FROM alpine
COPY --from=intermediate /project/target/x86_64-unknown-linux-musl/debug/ch10-sjlj /

CMD /ch10-sjlj
