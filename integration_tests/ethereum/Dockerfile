FROM node:14-alpine3.13

RUN apk update
RUN apk add --no-cache git python3 make build-base

COPY integration_tests/ethereum/package.json package.json
COPY integration_tests/ethereum/yarn.lock yarn.lock

RUN yarn install --production=false
RUN npm config set user 0

COPY integration_tests/ethereum/ .

COPY solidity/contracts/Gravity.sol contracts/Gravity.sol
COPY solidity/contracts/CosmosToken.sol contracts/CosmosToken.sol
COPY solidity/contracts/ICosmosToken.sol contracts/ICosmosToken.sol


ENV ARCHIVE_NODE_URL=""
EXPOSE 8545

RUN yarn run compile

CMD yarn start
