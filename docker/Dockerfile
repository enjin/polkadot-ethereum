FROM parity/polkadot:latest as origin
FROM debian:buster-slim
COPY --from=origin /usr/local/bin/polkadot /usr/local/bin
RUN apt-get update && apt-get install -y netcat
