module github.com/peggyjv/gravity-bridge

go 1.15

require (
	github.com/cosmos/cosmos-sdk v0.46.0
	github.com/cosmos/go-bip39 v1.0.0
	github.com/ethereum/go-ethereum v1.10.17
	github.com/miguelmota/go-ethereum-hdwallet v0.1.1
	github.com/ory/dockertest/v3 v3.9.1
	github.com/peggyjv/gravity-bridge/module/v2 v2.0.0-20220414231624-592368d8e8e1
	github.com/spf13/viper v1.12.0
	github.com/stretchr/testify v1.8.0
	github.com/tendermint/tendermint v0.34.20
)

replace (
	github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1
	github.com/peggyjv/gravity-bridge/module/v2 => github.com/crypto-org-chain/gravity-bridge/module/v2 v2.0.1-0.20221016022038-e581164e2889
)
