package keeper

import (
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
	"math/big"
	"testing"

	sdktypes "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestDetectMaliciousSupply(t *testing.T) {
	input := CreateTestEnv(t)

	// set supply to maximum value
	var testBigInt big.Int
	testBigInt.SetBit(new(big.Int), 256, 1).Sub(&testBigInt, big.NewInt(1))
	bigCoinAmount := sdktypes.NewIntFromBigInt(&testBigInt)

	err := input.GravityKeeper.DetectMaliciousSupply(input.Context, "stake", bigCoinAmount)
	require.Error(t, err, "didn't error out on too much added supply")
}

func TestDeleteExistingBatch_NoError(t *testing.T) {
	input := CreateTestEnv(t)

	otx := &types.BatchTx{
		BatchNonce:    2,
		Timeout:       100,
		Transactions:  []*types.SendToEthereum{},
		TokenContract: "0xC1B37f2abDb778f540fA5Db8e1fD2eADFC9a05Ed",
		Height:        10,
	}

	input.GravityKeeper.SetOutgoingTx(input.Context, otx)

	msg := &types.BatchExecutedEvent{
		TokenContract:  "0xC1B37f2abDb778f540fA5Db8e1fD2eADFC9a05Ed",
		EventNonce:     2,
		EthereumHeight: 100,
		BatchNonce:     2,
	}

	err := input.GravityKeeper.Handle(input.Context, msg)
	require.NoError(t, err)
}

func TestDeleteNonExistingBatch_ReturnError(t *testing.T) {
	input := CreateTestEnv(t)
	msg := &types.BatchExecutedEvent{
		TokenContract:  "0xC1B37f2abDb778f540fA5Db8e1fD2eADFC9a05Ed",
		EventNonce:     2,
		EthereumHeight: 100,
		BatchNonce:     2,
	}

	err := input.GravityKeeper.Handle(input.Context, msg)
	require.Error(t, err)
}
