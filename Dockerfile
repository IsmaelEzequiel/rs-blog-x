FROM rust:1.71.0-nightly

WORKDIR .
COPY . .

RUN /bin/bash -c './build.sh'

CMD ["blog"]