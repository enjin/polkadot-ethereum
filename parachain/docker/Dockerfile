FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Artemis. Here we create the binary."

ARG PROFILE=release
WORKDIR /artemis

COPY . /artemis

RUN cargo build --$PROFILE --no-default-features --features with-local-runtime

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Artemis binary."
ARG PROFILE=release
USER root
COPY --from=builder /artemis/target/$PROFILE/artemis /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /artemis artemis && \
	mkdir -p /artemis/.local/share && \
	mkdir /data && \
	chown -R artemis:artemis /data && \
	ln -s /data /artemis/.local/share/artemis && \
	rm -rf /usr/bin /usr/sbin

USER artemis
EXPOSE 30333 9933 9944
VOLUME ["/data"]
COPY ./docker/rococo-local.json .
COPY ./docker/spec.json .

EXPOSE 30333 9933 9944
RUN /usr/local/bin/artemis export-genesis-state --parachain-id=200 --chain=spec.json
