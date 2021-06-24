FROM efinity/polkadot:latest as origin

# Building from debian to be able to install netcat
FROM debian:buster-slim
COPY --from=origin /usr/local/bin/polkadot /usr/local/bin
RUN apt-get update && apt-get install -y netcat

EXPOSE 3033 9933 9944
ENTRYPOINT ["/bin/sh", "-c"]
