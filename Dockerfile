FROM rust:1.47 as builder
WORKDIR /usr/src/service

RUN rustup component add rustfmt

COPY Cargo.* ./
COPY ./src ./src

RUN cargo install --path .


FROM debian:stretch
WORKDIR /usr/www/app

# libssl.so is required by someone
RUN apt-get update && apt-get install -y curl openssl libssl-dev
# RUN curl -ks 'https://cert.host.server/ssl_certs/EnterpriseRootCA.crt' -o '/usr/local/share/ca-certificates/EnterpriseRootCA.crt'
RUN /usr/sbin/update-ca-certificates

COPY --from=builder /usr/local/cargo/bin/* ./

CMD ["./api"]
