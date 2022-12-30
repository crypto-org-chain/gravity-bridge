package simulation

import (
	"bytes"
	"encoding/binary"
	"fmt"

	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/kv"
	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
)

// NewDecodeStore returns a decoder function closure that unmarshals the KVPair's
// Value to the corresponding gravity type.
func NewDecodeStore(cdc codec.Codec) func(kvA, kvB kv.Pair) string {
	return func(kvA, kvB kv.Pair) string {
		switch {
		case bytes.Equal(kvA.Key[:1], []byte{types.ValidatorEthereumAddressKey}):
			return fmt.Sprintf("%v\n%v", common.BytesToAddress(kvA.Value), common.BytesToAddress(kvB.Value))

		case bytes.Equal(kvA.Key[:1], []byte{types.OrchestratorValidatorAddressKey}):
			return fmt.Sprintf("%v\n%v", sdk.ValAddress(kvA.Value), sdk.ValAddress(kvB.Value))

		case bytes.Equal(kvA.Key[:1], []byte{types.EthereumOrchestratorAddressKey}):
			return fmt.Sprintf("%v\n%v", sdk.AccAddress(kvA.Value), sdk.AccAddress(kvB.Value))

		case bytes.Equal(kvA.Key[:1], []byte{types.EthereumSignatureKey}):
			return fmt.Sprintf("%v\n%v", kvA.Value, kvB.Value)

		case bytes.Equal(kvA.Key[:1], []byte{types.EthereumEventVoteRecordKey}):
			var recordA, recordB types.EthereumEventVoteRecord
			cdc.MustUnmarshal(kvA.Value, &recordA)
			cdc.MustUnmarshal(kvB.Value, &recordB)
			return fmt.Sprintf("%v\n%v", recordA, recordB)

		case bytes.Equal(kvA.Key[:1], []byte{types.OutgoingTxKey}):
			var txA, txB types.OutgoingTx
			cdc.UnmarshalInterface(kvA.Value, &txA)
			cdc.UnmarshalInterface(kvB.Value, &txB)
			return fmt.Sprintf("%v\n%v", txA, txB)

		case bytes.Equal(kvA.Key[:1], []byte{types.SendToEthereumKey}):
			var sendA, sendB types.SendToEthereum
			cdc.MustUnmarshal(kvA.Value, &sendA)
			cdc.MustUnmarshal(kvB.Value, &sendB)
			return fmt.Sprintf("%v\n%v", sendA, sendB)

		case bytes.Equal(kvA.Key[:1], []byte{types.LastEventNonceByValidatorKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastObservedEventNonceKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LatestSignerSetTxNonceKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastSlashedOutgoingTxBlockKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastSlashedSignerSetTxNonceKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastSlashedOutgoingTxBlockKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastOutgoingBatchNonceKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastSendToEthereumIDKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastUnBondingBlockHeightKey}):
			return fmt.Sprintf("%v\n%v", binary.BigEndian.Uint64(kvA.Value), binary.BigEndian.Uint64(kvB.Value))

		case bytes.Equal(kvA.Key[:1], []byte{types.DenomToERC20Key}):
			return fmt.Sprintf("%v\n%v", common.BytesToAddress(kvA.Value), common.BytesToAddress(kvB.Value))

		case bytes.Equal(kvA.Key[:1], []byte{types.ERC20ToDenomKey}):
			return fmt.Sprintf("%v\n%v", string(kvA.Value), string(kvB.Value))

		case bytes.Equal(kvA.Key[:1], []byte{types.LastObservedSignerSetKey}):
			var setA, setB types.SignerSetTx
			cdc.MustUnmarshal(kvA.Value, &setA)
			cdc.MustUnmarshal(kvB.Value, &setB)
			return fmt.Sprintf("%v\n%v", setA, setB)

		case bytes.Equal(kvA.Key[:1], []byte{types.EthereumHeightVoteKey}),
			bytes.Equal(kvA.Key[:1], []byte{types.LastEthereumBlockHeightKey}):
			var heightA, heightB types.LatestEthereumBlockHeight

			cdc.MustUnmarshal(kvA.Value, &heightA)
			cdc.MustUnmarshal(kvB.Value, &heightB)

			return fmt.Sprintf("%v\n%v", heightA, heightB)
		default:
			panic(fmt.Sprintf("invalid staking key prefix %X", kvA.Key[:1]))
		}
	}
}
