from rust:1-stretch
RUN mkdir /build /app
WORKDIR /build
COPY . /build
RUN cargo build --release
RUN cp /build/target/release/alpaca-bot-api /app
WORKDIR /app
CMD ./alpaca-bot-api