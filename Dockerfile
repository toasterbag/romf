FROM ubuntu:20.04
COPY ./target/release/romf /usr/local/bin/romf
CMD ["romf"]