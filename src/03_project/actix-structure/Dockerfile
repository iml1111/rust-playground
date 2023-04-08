# Reference: https://www.codefeetime.com/post/docker-config-for-actix-web-diesel-and-postgres/

FROM rust:1.68 as build

WORKDIR /usr/src/imactor
COPY . .
RUN cargo install --path .
CMD ["/bin/sh"]

FROM gcr.io/distroless/cc-debian11

ARG ARCH=aarch64
COPY --from=build /usr/local/cargo/bin/imactor /usr/local/bin/imactor
COPY --from=build /usr/src/imactor/.env /.env
EXPOSE 8080
CMD ["imactor"]