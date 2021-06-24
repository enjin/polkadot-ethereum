FROM golang:1.16-stretch
WORKDIR /opt/relayer
ADD . .
RUN go build -v -o build/artemis-relay main.go

FROM parity/subkey:2.0.1
USER root
RUN apt-get update && apt-get install -y netcat
COPY --from=0 /opt/relayer/build/artemis-relay /usr/local/bin/
COPY --from=0 /opt/relayer/config.toml .
COPY --from=0 /opt/relayer/wait-for .

ENTRYPOINT ["/usr/local/bin/artemis-relay"]
