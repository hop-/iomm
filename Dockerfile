FROM rust:1.59-buster

WORKDIR /usr/src/iomm
COPY . .

RUN cargo install --path .

CMD ["iomm"]