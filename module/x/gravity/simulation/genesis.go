package simulation

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/rand"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
)

const (
	paramsKey                 = "params"
	lastObservedEventNonceKey = "last_observed_event_nonce"
	erc20ToDenomsKey          = "erc20_to_denoms"
)

func genRandomString(r *rand.Rand, minLength, maxLength uint8) string {
	if maxLength < minLength {
		panic(fmt.Errorf("maxLength should be greater than minLength"))
	}
	length := r.Intn(int(maxLength+1-minLength)) + int(minLength)
	bz := make([]byte, length)
	r.Read(bz)
	return hex.EncodeToString(bz)
}

func genRandomParams(r *rand.Rand) types.Params {
	return types.Params{
		GravityId:                                 genRandomString(r, 0, 32),
		ContractSourceHash:                        genRandomString(r, 0, uint8(r.Int())),
		BridgeEthereumAddress:                     common.HexToAddress(genRandomString(r, 0, uint8(r.Int()))).String(),
		BridgeChainId:                             r.Uint64(),
		SignedSignerSetTxsWindow:                  r.Uint64(),
		SignedBatchesWindow:                       r.Uint64(),
		EthereumSignaturesWindow:                  r.Uint64(),
		TargetEthTxTimeout:                        r.Uint64() + 60000,
		AverageBlockTime:                          r.Uint64() + 100,
		AverageEthereumBlockTime:                  r.Uint64() + 100,
		SlashFractionSignerSetTx:                  sdk.NewDec(r.Int63()),
		SlashFractionBatch:                        sdk.NewDec(r.Int63()),
		SlashFractionEthereumSignature:            sdk.NewDec(r.Int63()),
		SlashFractionConflictingEthereumSignature: sdk.NewDec(r.Int63()),
		UnbondSlashingSignerSetTxsWindow:          r.Uint64(),
		BridgeActive:                              r.Intn(2) > 0,
		BatchCreationPeriod:                       r.Uint64() + 1,
		BatchMaxElement:                           r.Uint64(),
		ObserveEthereumHeightPeriod:               r.Uint64() + 1,
	}
}

func RandomizedGenState(simState *module.SimulationState) {
	var (
		params                 types.Params
		lastObservedEventNonce uint64
		erc20ToDenoms          []*types.ERC20ToDenom
		// outgoingTxs                []*codectypes.Any
		// confirmations              []*codectypes.Any
		// ethereumEventVoteRecords   []*types.EthereumEventVoteRecord
		// delegateKeys               []*types.MsgDelegateKeys
		// unbatchedSendToEthereumTxs []*types.SendToEthereum
	)

	simState.AppParams.GetOrGenerate(
		simState.Cdc, paramsKey, &params, simState.Rand,
		func(r *rand.Rand) { params = genRandomParams(r) },
	)

	simState.AppParams.GetOrGenerate(
		simState.Cdc, lastObservedEventNonceKey, &lastObservedEventNonce, simState.Rand,
		func(r *rand.Rand) { lastObservedEventNonce = r.Uint64() },
	)

	simState.AppParams.GetOrGenerate(
		simState.Cdc, erc20ToDenomsKey, &erc20ToDenoms, simState.Rand,
		func(r *rand.Rand) {
			length := r.Intn(10) + 1
			for i := 0; i < length; i++ {
				erc20ToDenoms = append(
					erc20ToDenoms,
					&types.ERC20ToDenom{
						Erc20: randomEthAddress(r).String(),
						Denom: genRandomString(r, 1, 10),
					},
				)
			}
			erc20ToDenoms = append(erc20ToDenoms, &types.ERC20ToDenom{
				Erc20: randomEthAddress(r).String(),
				Denom: sdk.DefaultBondDenom,
			})
		},
	)

	gravityGenesis := &types.GenesisState{
		Params:                 &params,
		LastObservedEventNonce: lastObservedEventNonce,
		Erc20ToDenoms:          erc20ToDenoms,
	}

	bz, err := json.MarshalIndent(gravityGenesis, "", " ")
	if err != nil {
		panic(err)
	}
	fmt.Printf("Selected randomly generated %s parameters:\n%s\n", types.ModuleName, bz)

	simState.GenState[types.ModuleName] = simState.Cdc.MustMarshalJSON(gravityGenesis)
}
