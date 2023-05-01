FROM rust:1.69

WORKDIR /src/blog
COPY . .

RUN cargo install --path .

CMD ["blog"]