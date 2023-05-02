FROM rust:1.69

WORKDIR .
COPY . .

RUN cargo install --path .

CMD ["blog"]