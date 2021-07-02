# Building from debian to be able to install netcat
FROM node:buster-slim
COPY --from=efinity/polkadot:latest /usr/local/bin/polkadot /usr/local/bin
COPY polkadot-launch ./polkadot-launch
COPY launchConfigOverrides.json .
COPY config.json .
COPY create_spec.sh .
RUN apt-get update && apt-get install -y netcat jq && cd polkadot-launch && yarn && yarn build && cd ..

EXPOSE 3033 9933 9944
ENTRYPOINT ["/bin/sh", "-c"]
