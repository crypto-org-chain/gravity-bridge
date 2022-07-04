module github.com/peggyjv/gravity-bridge/testnet

go 1.16

require (
	github.com/BurntSushi/toml v1.1.0
	github.com/cosmos/cosmos-sdk v0.45.3
	github.com/cosmos/go-bip39 v1.0.0
	github.com/ethereum/go-ethereum v1.10.17
	github.com/matttproud/golang_protobuf_extensions v1.0.2-0.20181231171920-c182affec369 // indirect
	github.com/moby/term v0.0.0-20210610120745-9d4ed1856297 // indirect
	github.com/ory/dockertest/v3 v3.6.5
	github.com/peggyjv/gravity-bridge/module/v2 v2.0.0-00010101000000-000000000000
	github.com/stretchr/testify v1.7.1
	github.com/tendermint/tendermint v0.34.19
	gotest.tools/v3 v3.0.3 // indirect
)

replace github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1

replace github.com/peggyjv/gravity-bridge/module/v2 => ../module
