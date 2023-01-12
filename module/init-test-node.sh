#!/bin/bash

CHAINID="cronos_9000-1"
MONIKER="localtestnet"

# localKey address 0x7cb61d4117ae31a12e393a1cfa3bac666481d02e
VAL_KEY="localkey"
VAL_MNEMONIC="gesture inject test cycle original hollow east ridge hen combine junk child bacon zero hope comfort vacuum milk pitch cage oppose unhappy lunar seat"

# user1 address 0xc6fe5d33615a1c52c08018c47e8bc53646a0e101
USER1_KEY="user1"
USER1_MNEMONIC="night renew tonight dinner shaft scheme domain oppose echo summer broccoli agent face guitar surface belt veteran siren poem alcohol menu custom crunch index"

# user2 address 0x963ebdf2e1f8db8707d05fc75bfeffba1b5bac17
USER2_KEY="user2"
USER2_MNEMONIC="early inmate pudding three girl word crater strike party hunt item head stadium frozen raven that snap across canyon media quality dragon elder stereo"

# remove existing daemon and client
rm -rf ~/.gravity*

# Import keys from mnemonics
echo $VAL_MNEMONIC | ./build/gravity keys add $VAL_KEY --recover --keyring-backend test
echo $USER1_MNEMONIC | ./build/gravity keys add $USER1_KEY --recover --keyring-backend test
echo $USER2_MNEMONIC | ./build/gravity keys add $USER2_KEY --recover --keyring-backend test

./build/gravity init $MONIKER --chain-id $CHAINID

# Allocate genesis accounts (cosmos formatted addresses)
./build/gravity add-genesis-account "$(./build/gravity keys show $VAL_KEY -a --keyring-backend test)" 1000000000000000000000aphoton,1000000000000000000stake --keyring-backend test
./build/gravity add-genesis-account "$(./build/gravity keys show $USER1_KEY -a --keyring-backend test)" 1000000000000000000000aphoton,1000000000000000000stake --keyring-backend test
./build/gravity add-genesis-account "$(./build/gravity keys show $USER2_KEY -a --keyring-backend test)" 1000000000000000000000aphoton,1000000000000000000stake --keyring-backend test

# Sign genesis transaction
./build/gravity gentx $VAL_KEY 1000000000000000000stake 0xc1b37f2abdb778f540fa5db8e1fd2eadfc9a05ed cosmos1tctknvd48jf4ztsxpgfe85gkd493npn8rceepj --amount=1000000000000000000000aphoton --chain-id $CHAINID --keyring-backend test

# Collect genesis tx
./build/gravity collect-gentxs

# Run this to ensure everything worked and that the genesis file is setup correctly
./build/gravity validate-genesis

# Start the node (remove the --pruning=nothing flag if historical queries are not needed)
./build/gravity start --pruning=nothing --rpc.unsafe --log_level info --json-rpc.api eth,txpool,personal,net,debug,web3 --minimum-gas-prices 200000aphoton
