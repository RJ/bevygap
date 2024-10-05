FROM rust:1 as build-env
WORKDIR /app
COPY . /app


RUN cargo build --release -p bevygap_matchmaker

# Now that we've compiled the server, we copy the required bits into the
# small distroless image, which we'll use to run the server.

# the :debug version includes a busybox shell
FROM gcr.io/distroless/cc-debian12:debug
COPY --from=build-env /app/target/release/bevygap_matchmaker /app/

WORKDIR /app

ENTRYPOINT ["./bevygap_matchmaker"]