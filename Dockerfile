FROM rust:1.33 as build
COPY ./ ./
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN cp target/x86_64-unknown-linux-musl/release/https-redirect /build-out/
RUN ls /build-out/
FROM scratch
COPY --from=build /build-out/https-redirect /
# EXPOSE 9001
CMD ["/https-redirect"]
