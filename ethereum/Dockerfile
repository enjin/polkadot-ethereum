FROM trufflesuite/ganache-cli:latest as builder
FROM node:lts-buster
WORKDIR /app
RUN apt-get update && apt-get -y dist-upgrade
RUN apt-get -y install netcat
#RUN apk add --update --no-cache python3 make && ln -sf python3 /usr/bin/python
RUN npm install -g truffle --dotenv-extended --unsafe-perm=true --allow-root 
#RUN cp $(which truffle) .
#WORKDIR /apps
COPY . .
COPY --from=builder "/app/ganache-core.docker.cli.js" "./ganache-core.docker.cli.js"
COPY --from=builder "/app/ganache-core.docker.cli.js.map" "./ganache-core.docker.cli.js.map"
COPY env.template .env
ENV DOCKER true
RUN yarn install
#COPY --from=builder "/app/truffle" "./truffle"
ENTRYPOINT ["/bin/sh", "-c"]
