package simulation

import (
	"fmt"
	"math/rand"
	"time"

	"cosmossdk.io/math"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/codec"
	simappparams "github.com/cosmos/cosmos-sdk/simapp/params"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/keeper"
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
)

// Simulation operation weights constants
//
//nolint:gosec // these are not hardcoded credentials.
const (
	OpWeightMsgDeleteKeys                   = "op_weight_delegate_keys"
	OpWeightMsgSubmitEthereumTxConfirmation = "op_weight_msg_submit_ethereum_tx_confirmation"
	OpWeightMsgSubmitEthereumEvent          = "op_weight_msg_submit_ethereum_event"
	OpWeightMsgSendToEthereum               = "op_weight_msg_send_to_ethereum"
	OpWeightMsgRequestBatchTx               = "op_weight_msg_request_batch_tx"
	OpWeightMsgCancelSendToEthereum         = "op_weight_msg_cancel_send_to_ethereum"
	OpWeightMsgEthereumHeightVote           = "op_weight_msg_ethereum_height_vote"

	DefaultWeight  = 50
	maxWaitSeconds = 10
)

// Gravity message types and routes
var (
	TypeMsgDeleteKeys                   = sdk.MsgTypeURL(&types.MsgDelegateKeys{})
	TypeMsgSubmitEthereumTxConfirmation = sdk.MsgTypeURL(&types.MsgSubmitEthereumTxConfirmation{})
	TypeMsgSubmitEthereumEvent          = sdk.MsgTypeURL(&types.MsgSubmitEthereumEvent{})
	TypeMsgMsgSendToEthereum            = sdk.MsgTypeURL(&types.MsgSendToEthereum{})
	TypeMsgRequestBatchTx               = sdk.MsgTypeURL(&types.MsgRequestBatchTx{})
	TypeMsgCancelSendToEthereum         = sdk.MsgTypeURL(&types.MsgCancelSendToEthereum{})
	TypeMsgEthereumHeightVote           = sdk.MsgTypeURL(&types.MsgEthereumHeightVote{})

	eventTypes = []string{"SendToCosmosEvent", "BatchExecutedEvent", "ContractCallExecutedEvent", "ERC20DeployedEvent", "SignerSetTxExecutedEvent"}
	txTypes    = []string{"SignerSetTx", "BatchTx"}
)

type simulateContext struct {
	gk  keeper.Keeper
	ak  types.AccountKeeper
	bk  types.BankKeeper
	cdc codec.JSONCodec
}

type opGenerator func(*simulateContext) simtypes.Operation

// WeightedOperations returns all the operations from the module with their respective weights
func WeightedOperations(
	appParams simtypes.AppParams, cdc codec.JSONCodec, gk keeper.Keeper, ak types.AccountKeeper, bk types.BankKeeper,
) simulation.WeightedOperations {
	var ops simulation.WeightedOperations
	opFuncs := map[string]opGenerator{
		OpWeightMsgDeleteKeys:                   SimulateMsgDeleteKeys,
		OpWeightMsgSubmitEthereumTxConfirmation: SimulateMsgSubmitEthereumTxConfirmation,
		OpWeightMsgSubmitEthereumEvent:          SimulateMsgSubmitEthereumEvent,
		OpWeightMsgSendToEthereum:               SimulateMsgSendToEthereum,
		OpWeightMsgRequestBatchTx:               SimulateMsgRequestBatchTx,
		OpWeightMsgCancelSendToEthereum:         SimulateMsgCancelSendToEthereum,
		OpWeightMsgEthereumHeightVote:           SimulateMsgEthereumHeightVote,
	}

	simCtx := &simulateContext{
		gk:  gk,
		ak:  ak,
		bk:  bk,
		cdc: cdc,
	}

	for k, f := range opFuncs {
		var v int
		appParams.GetOrGenerate(cdc, k, &v, nil,
			func(_ *rand.Rand) {
				v = DefaultWeight
			},
		)
		ops = append(
			ops,
			simulation.NewWeightedOperation(
				v,
				f(simCtx),
			),
		)
	}
	return ops
}

func SimulateMsgDeleteKeys(simCtx *simulateContext) simtypes.Operation {
	return func(
		r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context,
		accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		ak := simCtx.ak
		bk := simCtx.bk
		sk := simCtx.gk.StakingKeeper
		cdc := simCtx.cdc

		val, err := randomValidator(ctx, r, sk, accs)
		if err != nil {
			return simtypes.OperationMsg{}, nil, fmt.Errorf("none of the sim account is a validator")
		}
		valAddr := sdk.ValAddress(val.Address).String()
		orchAddr := val.Address.String()
		ethAddr := common.BytesToAddress(val.Address.Bytes()).String()
		nonce, err := ak.GetSequence(ctx, val.Address)
		if err != nil {
			return simtypes.OperationMsg{}, nil, err
		}

		bz := cdc.MustMarshalJSON(&types.DelegateKeysSignMsg{
			ValidatorAddress: valAddr,
			Nonce:            nonce,
		})

		hash := crypto.Keccak256Hash(bz).Bytes()
		ethPrivKey, err := crypto.ToECDSA(val.PrivKey.Bytes())
		if err != nil {
			return simtypes.OperationMsg{}, nil, err
		}
		sig, err := types.NewEthereumSignature(hash, ethPrivKey)
		if err != nil {
			return simtypes.OperationMsg{}, nil, err
		}

		msg := &types.MsgDelegateKeys{
			ValidatorAddress:    valAddr,
			OrchestratorAddress: orchAddr,
			EthereumAddress:     ethAddr,
			EthSignature:        sig,
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      val,
			AccountKeeper:   ak,
			Bankkeeper:      bk,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.NewCoins(),
		}

		oper, ops, err := simulation.GenAndDeliverTxWithRandFees(txCtx)
		return oper, ops, err
	}
}

func SimulateMsgSubmitEthereumTxConfirmation(simCtx *simulateContext) simtypes.Operation {
	return simulateMsgSubmitEthereumTxConfirmation(simCtx)
}

func SimulateMsgSubmitEthereumEvent(simCtx *simulateContext) simtypes.Operation {
	return simulateMsgSubmitEthereumEvent(simCtx)
}

func SimulateMsgSendToEthereum(simCtx *simulateContext) simtypes.Operation {
	return func(
		r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context,
		accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		ak := simCtx.ak
		bk := simCtx.bk

		acc, _ := simtypes.RandomAcc(r, accs)
		balance := bk.SpendableCoins(ctx, acc.Address)
		expendable := balance.AmountOf(sdk.DefaultBondDenom)
		amount := sdk.NewCoin(sdk.DefaultBondDenom, simtypes.RandomAmount(r, expendable))
		fee := sdk.NewCoin(sdk.DefaultBondDenom, simtypes.RandomAmount(r, expendable.Sub(amount.Amount)))

		msg := &types.MsgSendToEthereum{
			Sender:            acc.Address.String(),
			EthereumRecipient: randomEthAddress(r).String(),
			Amount:            amount,
			BridgeFee:         fee,
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      acc,
			AccountKeeper:   ak,
			Bankkeeper:      bk,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.Coins{amount.Add(fee)},
		}

		oper, ops, err := simulation.GenAndDeliverTxWithRandFees(txCtx)

		whenCall := ctx.BlockHeader().Time.Add(time.Duration(r.Intn(maxWaitSeconds)+1) * time.Second)
		ops = append(ops, simtypes.FutureOperation{
			BlockTime: whenCall,
			Op:        simulateMsgRequestBatchTx(simCtx),
		})
		return oper, ops, err
	}
}

func SimulateMsgRequestBatchTx(simCtx *simulateContext) simtypes.Operation {
	return simulateMsgRequestBatchTx(simCtx)
}

func SimulateMsgCancelSendToEthereum(simCtx *simulateContext) simtypes.Operation {
	return simulateMsgCancelSendToEthereum(simCtx)
}

func SimulateMsgEthereumHeightVote(simCtx *simulateContext) simtypes.Operation {
	return simulateMsgEthereumHeightVote(simCtx)
}

func simulateMsgSubmitEthereumTxConfirmation(simCtx *simulateContext) simtypes.Operation {
	return func(
		r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context,
		accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		ak := simCtx.ak
		bk := simCtx.bk
		gk := simCtx.gk

		orch, err := randomOrchetrator(ctx, r, gk, accs)
		if err != nil {
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "sim account is not orchestrator"), nil, err
		}
		orchAddr := orch.Address.String()

		ethPrivKey, err := crypto.ToECDSA(orch.PrivKey.Bytes())
		if err != nil {
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "can not get eth private key"), nil, err
		}
		ethAddr := common.BytesToAddress(orch.Address.Bytes()).String()

		var msg *types.MsgSubmitEthereumTxConfirmation

		txType := txTypes[r.Intn(len(txTypes))]
		switch txType {
		case "SignerSetTx":
			tx := gk.GetLatestSignerSetTx(ctx)

			if tx == nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, ""), nil, nil
			}

			gravityId := gk.GetParams(ctx).GravityId
			checkpoint := tx.GetCheckpoint([]byte(gravityId))
			signature, err := types.NewEthereumSignature(checkpoint, ethPrivKey)
			if err != nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "can not sign the msg"), nil, err
			}

			signerSetTxConfirmation := &types.SignerSetTxConfirmation{
				SignerSetNonce: tx.Nonce,
				EthereumSigner: ethAddr,
				Signature:      signature,
			}

			confirmation, err := types.PackConfirmation(signerSetTxConfirmation)
			if err != nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "failed to pack the confirmation"), nil, err
			}

			msg = &types.MsgSubmitEthereumTxConfirmation{
				Confirmation: confirmation,
				Signer:       orchAddr,
			}

		case "BatchTx":
			_, tokenContract, err := gk.DenomToERC20Lookup(ctx, sdk.DefaultBondDenom)
			if err != nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "can not find token contract"), nil, err
			}
			resp, err := gk.LastBatchTx(ctx, &types.LastBatchTxRequest{TokenContract: tokenContract.Hex()})
			if err != nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "can not fetch the last batch tx"), nil, err
			}
			batch := resp.Batch
			if batch == nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "there is no batchtx"), nil, err
			}

			gravityId := gk.GetParams(ctx).GravityId
			checkpoint := batch.GetCheckpoint([]byte(gravityId))
			signature, err := types.NewEthereumSignature(checkpoint, ethPrivKey)
			if err != nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "can not sign the msg"), nil, err
			}

			batchTxConfirmation := &types.BatchTxConfirmation{
				TokenContract:  batch.TokenContract,
				BatchNonce:     batch.BatchNonce,
				EthereumSigner: ethAddr,
				Signature:      signature,
			}

			confirmation, err := types.PackConfirmation(batchTxConfirmation)
			if err != nil {
				return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "failed to pack the confirmation"), nil, err
			}

			msg = &types.MsgSubmitEthereumTxConfirmation{
				Confirmation: confirmation,
				Signer:       orchAddr,
			}

		default:
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumTxConfirmation, "not supported tx type"), nil, err
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      orch,
			AccountKeeper:   ak,
			Bankkeeper:      bk,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.Coins{},
		}

		oper, ops, err := simulation.GenAndDeliverTxWithRandFees(txCtx)
		return oper, ops, err
	}
}

func simulateMsgSubmitEthereumEvent(simCtx *simulateContext) simtypes.Operation {
	return func(
		r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context,
		accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		ak := simCtx.ak
		bk := simCtx.bk
		gk := simCtx.gk

		orch, err := randomOrchetrator(ctx, r, gk, accs)
		if err != nil {
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumEvent, "sim account is not orchestrator"), nil, err
		}
		orchAddr := orch.Address.String()

		var event types.EthereumEvent
		eventType := eventTypes[r.Intn(len(eventTypes))]
		switch eventType {
		case "SendToCosmosEvent":
			event = &types.SendToCosmosEvent{
				EventNonce:     1,
				TokenContract:  randomEthAddress(r).String(),
				Amount:         simtypes.RandomAmount(r, math.NewIntFromUint64(1e18)),
				EthereumSender: randomEthAddress(r).String(),
				CosmosReceiver: simtypes.RandomAccounts(r, 1)[0].Address.String(),
				EthereumHeight: uint64(r.Int63n(1e10)),
			}

		case "BatchExecutedEvent":
			event = &types.BatchExecutedEvent{
				TokenContract:  randomEthAddress(r).String(),
				EventNonce:     uint64(r.Int63n(1e10)),
				EthereumHeight: uint64(r.Int63n(1e10)),
				BatchNonce:     uint64(r.Int63n(1e10)),
			}

		case "ContractCallExecutedEvent":
			event = &types.ContractCallExecutedEvent{
				EventNonce:        uint64(r.Int63n(1e10)),
				InvalidationNonce: uint64(r.Int63n(1e10)),
				EthereumHeight:    uint64(r.Int63n(1e10)),
			}

		case "ERC20DeployedEvent":
			event = &types.ERC20DeployedEvent{
				EventNonce:    uint64(r.Int63n(1e10)),
				CosmosDenom:   sdk.DefaultBondDenom,
				TokenContract: randomEthAddress(r).String(),
			}

		case "SignerSetTxExecutedEvent":
			event = &types.SignerSetTxExecutedEvent{
				EventNonce:       uint64(r.Int63n(1e10)),
				SignerSetTxNonce: uint64(r.Int63n(1e10)),
				EthereumHeight:   uint64(r.Int63n(1e10)),
				Members: []*types.EthereumSigner{
					{
						Power:           uint64(r.Int63n(1e10)),
						EthereumAddress: randomEthAddress(r).String(),
					},
				},
			}
		default:
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumEvent, "not supported event type"), nil, err
		}

		packed, err := types.PackEvent(event)
		if err != nil {
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgSubmitEthereumEvent, "can not pack eth event"), nil, err
		}

		msg := &types.MsgSubmitEthereumEvent{
			Event:  packed,
			Signer: orchAddr,
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      orch,
			AccountKeeper:   ak,
			Bankkeeper:      bk,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.Coins{},
		}

		oper, ops, err := simulation.GenAndDeliverTxWithRandFees(txCtx)
		return oper, ops, err
	}
}

func simulateMsgRequestBatchTx(simCtx *simulateContext) simtypes.Operation {
	return func(
		r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context,
		accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		ak := simCtx.ak
		bk := simCtx.bk
		gk := simCtx.gk

		orch, err := randomOrchetrator(ctx, r, gk, accs)
		if err != nil {
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgRequestBatchTx, "sim account is not orchestrator"), nil, err
		}
		orchAddr := orch.Address.String()
		msg := &types.MsgRequestBatchTx{
			Signer: orchAddr,
			Denom:  sdk.DefaultBondDenom,
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      orch,
			AccountKeeper:   ak,
			Bankkeeper:      bk,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.Coins{},
		}

		oper, ops, err := simulation.GenAndDeliverTxWithRandFees(txCtx)
		return oper, ops, err
	}
}

func simulateMsgCancelSendToEthereum(simCtx *simulateContext) simtypes.Operation {
	return func(
		r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context,
		accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		ak := simCtx.ak
		bk := simCtx.bk
		gk := simCtx.gk

		sender, id, err := randomSenderAndID(ctx, r, gk, accs)
		if err != nil {
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgCancelSendToEthereum, "no sender found"), nil, err
		}

		msg := &types.MsgCancelSendToEthereum{
			Id:     id,
			Sender: sender.Address.String(),
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      sender,
			AccountKeeper:   ak,
			Bankkeeper:      bk,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.Coins{},
		}

		oper, ops, err := simulation.GenAndDeliverTxWithRandFees(txCtx)
		return oper, ops, err
	}
}

func simulateMsgEthereumHeightVote(simCtx *simulateContext) simtypes.Operation {
	return func(
		r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context,
		accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		ak := simCtx.ak
		bk := simCtx.bk
		gk := simCtx.gk

		orch, err := randomOrchetrator(ctx, r, gk, accs)
		if err != nil {
			return simtypes.NoOpMsg(types.ModuleName, TypeMsgRequestBatchTx, "sim account is not orchestrator"), nil, err
		}
		orchAddr := orch.Address.String()

		msg := &types.MsgEthereumHeightVote{
			EthereumHeight: r.Uint64(),
			Signer:         orchAddr,
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      orch,
			AccountKeeper:   ak,
			Bankkeeper:      bk,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.Coins{},
		}

		oper, ops, err := simulation.GenAndDeliverTxWithRandFees(txCtx)
		return oper, ops, err
	}
}

func randomEthAddress(r *rand.Rand) common.Address {
	addr := make([]byte, 32)
	_, err := r.Read(addr)
	if err != nil {
		panic(err)
	}
	return common.BytesToAddress(addr)
}

func randomValidator(ctx sdk.Context, r *rand.Rand, sk types.StakingKeeper, accs []simtypes.Account) (simtypes.Account, error) {
	var valset []simtypes.Account
	for _, acc := range accs {
		val := sdk.ValAddress(acc.Address)
		if sk.Validator(ctx, val) != nil {
			valset = append(valset, acc)
		}
	}
	if len(valset) == 0 {
		return simtypes.Account{}, fmt.Errorf("none of the sim account is a validator")
	}
	val, _ := simtypes.RandomAcc(r, valset)
	return val, nil
}

func randomOrchetrator(ctx sdk.Context, r *rand.Rand, gk keeper.Keeper, accs []simtypes.Account) (simtypes.Account, error) {
	var orchSet []simtypes.Account
	for _, acc := range accs {
		val := sdk.ValAddress(acc.Address)
		if gk.StakingKeeper.Validator(ctx, val) != nil && gk.GetOrchestratorValidatorAddress(ctx, acc.Address) != nil {
			orchSet = append(orchSet, acc)
		}
	}
	if len(orchSet) == 0 {
		return simtypes.Account{}, fmt.Errorf("none of the sim account is both a validator and orchestrator")
	}
	val, _ := simtypes.RandomAcc(r, orchSet)
	return val, nil
}

func randomSenderAndID(ctx sdk.Context, r *rand.Rand, gk keeper.Keeper, accs []simtypes.Account) (simtypes.Account, uint64, error) {
	var senders []simtypes.Account
	var ids map[string][]uint64
	for _, acc := range accs {
		sender := acc.Address.String()
		resp, err := gk.UnbatchedSendToEthereums(ctx, &types.UnbatchedSendToEthereumsRequest{SenderAddress: sender, Pagination: nil})
		if err != nil {
			return simtypes.Account{}, 0, err
		}
		if len(resp.SendToEthereums) == 0 {
			continue
		}
		senders = append(senders, acc)
		for _, tx := range resp.SendToEthereums {
			ids[sender] = append(ids[sender], tx.Id)
		}
	}
	sender, _ := simtypes.RandomAcc(r, accs)
	senderIDs := ids[sender.Address.String()]
	id := senderIDs[r.Intn(len(senderIDs))]
	return sender, id, nil
}
