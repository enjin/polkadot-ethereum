FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Artemis. Here we create the binary."

ARG PROFILE=release
WORKDIR /artemis

COPY . /artemis

RUN cargo build --$PROFILE --no-default-features --features with-local-runtime

# ===== SECOND STAGE ======

FROM node:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Artemis binary."
ARG PROFILE=release
USER root
COPY --from=builder /artemis/target/$PROFILE/artemis /usr/local/bin


RUN apt-get update && apt-get install -y netcat curl

COPY ./docker/rococo-local.json .
COPY ./docker/spec.json .
COPY ./docker/wait-for.sh .
COPY ./docker/update_spec.sh .
COPY ./docker/overrideParachainSpec.js .
COPY ./docker/transformEthHeader.js .
COPY ./docker/package.json .
RUN npm install

EXPOSE 30333 9933 9944
VOLUME ["/data"]

EXPOSE 30333 9933 9944
