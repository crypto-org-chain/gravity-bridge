module github.com/peggyjv/gravity-bridge/testnet

go 1.19

require (
	github.com/BurntSushi/toml v1.2.0
	github.com/cosmos/cosmos-sdk v0.47.0-rc1
	github.com/cosmos/go-bip39 v1.0.0
	github.com/ethereum/go-ethereum v1.10.26
	github.com/ory/dockertest/v3 v3.9.1
	github.com/peggyjv/gravity-bridge/module/v2 v2.0.0-00010101000000-000000000000
	github.com/stretchr/testify v1.8.1
	github.com/tendermint/tendermint v0.37.0-rc2
)

replace github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1

replace github.com/peggyjv/gravity-bridge/module/v2 => ../module
