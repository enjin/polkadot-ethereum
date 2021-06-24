FROM trufflesuite/ganache-cli:latest as builder

# Building from node image to be able to run netcat
FROM node:buster-slim
WORKDIR /app

# Extra scripts for docker-compose
COPY docker/deploy_contracts.sh .
COPY docker/restart_ganache.sh .
COPY docker/start_ganache.sh .
COPY docker/wait-for .
COPY docker/truffle-config.js .
COPY package.json  .
COPY scripts scripts
COPY contracts contracts
COPY migrations migrations

COPY --from=builder "/app/ganache-core.docker.cli.js" "./ganache-core.docker.cli.js"
COPY --from=builder "/app/ganache-core.docker.cli.js.map" "./ganache-core.docker.cli.js.map"
COPY env.template .env

# We need this so that ganache-cli exposes itself externally
ENV DOCKER true
RUN apt-get update && apt-get -y upgrade && apt-get -y install netcat git procps && \
    npm install -g truffle --dotenv-extended --unsafe-perm=true --allow-root && \
    yarn install

EXPOSE 8545
ENTRYPOINT ["/bin/sh", "-c"]
