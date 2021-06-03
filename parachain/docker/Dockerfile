FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Artemis. Here we create the binary."

ARG PROFILE=release
WORKDIR /artemis

COPY . /artemis

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Artemis binary."
ARG PROFILE=release
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

#ENTRYPOINT ["/usr/local/bin/artemis"]
