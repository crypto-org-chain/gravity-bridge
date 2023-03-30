#!/bin/bash

CHAINID="simulation-app"
MONIKER="localtestnet"

# localKey address 0x7cb61d4117ae31a12e393a1cfa3bac666481d02e
VAL_KEY="localkey"
VAL_MNEMONIC="gesture inject test cycle original hollow east ridge hen combine junk child bacon zero hope comfort vacuum milk pitch cage oppose unhappy lunar seat"
COSMOS_VAL_ADDR="cosmos1dd6psq88kntuzyap944et8fmh0mxmw2wqh9emp"

# user1 address 0xc6fe5d33615a1c52c08018c47e8bc53646a0e101
USER1_KEY="user1"
USER1_MNEMONIC="night renew tonight dinner shaft scheme domain oppose echo summer broccoli agent face guitar surface belt veteran siren poem alcohol menu custom crunch index"

# user2 address 0x963ebdf2e1f8db8707d05fc75bfeffba1b5bac17
USER2_KEY="user2"
USER2_MNEMONIC="early inmate pudding three girl word crater strike party hunt item head stadium frozen raven that snap across canyon media quality dragon elder stereo"

# remove existing daemon and client
rm -rf ~/.gravity*

# Import keys from mnemonics
echo $VAL_MNEMONIC | ./module/build/gravity keys add $VAL_KEY --recover --keyring-backend test
echo $USER1_MNEMONIC | ./module/build/gravity keys add $USER1_KEY --recover --keyring-backend test
echo $USER2_MNEMONIC | ./module/build/gravity keys add $USER2_KEY --recover --keyring-backend test

./module/build/gravity init $MONIKER --chain-id $CHAINID

# Allocate genesis accounts (cosmos formatted addresses)
./module/build/gravity add-genesis-account "$(./module/build/gravity keys show $VAL_KEY -a --keyring-backend test)" 1000000000000000000000aphoton,1000000000000000000stake --keyring-backend test
./module/build/gravity add-genesis-account "$(./module/build/gravity keys show $USER1_KEY -a --keyring-backend test)" 1000000000000000000000aphoton,1000000000000000000stake --keyring-backend test
./module/build/gravity add-genesis-account "$(./module/build/gravity keys show $USER2_KEY -a --keyring-backend test)" 1000000000000000000000aphoton,1000000000000000000stake --keyring-backend test


ETH_ADDR="0x2ad5feff5dc714dd9aa5f7f4f252a2f3246c2de9"
SIG="0x724c8a35181bde7878268635070c6411c30a94b106e3f50b8736c62e8f4399c5472be83c692210b2779a62f39f4383fc036daa4d66fe35034c9485ff6a68ba0e1b"
# Sign genesis transaction
./module/build/gravity gentx $VAL_KEY 1000000000000000000stake $ETH_ADDR $COSMOS_VAL_ADDR $SIG --amount=1000000000000000000000aphoton --chain-id $CHAINID --keyring-backend test

# Collect genesis tx
./module/build/gravity collect-gentxs

# Run this to ensure everything worked and that the genesis file is setup correctly
./module/build/gravity validate-genesis

# Start the node (remove the --pruning=nothing flag if historical queries are not needed)
./module/build/gravity start --pruning=nothing --rpc.unsafe --log_level info --minimum-gas-prices 200000aphoton