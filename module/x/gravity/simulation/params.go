package simulation

import (
	"fmt"
	"math/rand"

	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
)

const (
	keySignedSignerSetTxsWindow = "SignedSignerSetTxWindow"
	keySignedBatchesWindow      = "SignedBatchesWindow"
	keyEthereumSignaturesWindow = "EthereumSignaturesWindow"
	keyBatchCreationPeriod      = "BatchCreationPeriod"
	keyBatchMaxElement          = "BatchMaxElement"
)

// ParamChanges defines the parameters that can be modified by param change proposals
// on the simulation
func ParamChanges(r *rand.Rand) []simtypes.ParamChange {
	return []simtypes.ParamChange{
		simulation.NewSimParamChange(types.ModuleName, keySignedSignerSetTxsWindow,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimParamChange(types.ModuleName, keySignedBatchesWindow,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimParamChange(types.ModuleName, keyEthereumSignaturesWindow,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimParamChange(types.ModuleName, keyBatchCreationPeriod,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimParamChange(types.ModuleName, keyBatchMaxElement,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(100))+1)
			},
		),
	}
}
