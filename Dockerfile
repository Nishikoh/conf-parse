FROM rust:1.60 as base

WORKDIR /app

COPY Cargo.toml Cargo.toml
RUN mkdir src

# docker cacheを効かせるため
# TODO:sccache, mold なんかを使うとbuild速度改善できるかも
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
RUN rm -rf target/release/deps/parse_conf*

FROM base as develop
COPY test.conf test.conf
CMD cargo run test.conf

FROM base as build-production
RUN cargo build --release

FROM gcr.io/distroless/cc as production
COPY --from=build-production /app/target/release/parse-conf /
COPY test.conf test.conf
ENTRYPOINT [ "./parse-conf" ]
CMD ["test.conf"]