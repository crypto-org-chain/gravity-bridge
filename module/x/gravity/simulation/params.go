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
func ParamChanges(r *rand.Rand) []simtypes.LegacyParamChange {
	return []simtypes.LegacyParamChange{
		simulation.NewSimLegacyParamChange(types.ModuleName, keySignedSignerSetTxsWindow,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimLegacyParamChange(types.ModuleName, keySignedBatchesWindow,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimLegacyParamChange(types.ModuleName, keyEthereumSignaturesWindow,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimLegacyParamChange(types.ModuleName, keyBatchCreationPeriod,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(maxBlocksInOneRound))+1)
			},
		),
		simulation.NewSimLegacyParamChange(types.ModuleName, keyBatchMaxElement,
			func(r *rand.Rand) string {
				return fmt.Sprintf("\"%d\"", uint64(r.Intn(100))+1)
			},
		),
	}
}
