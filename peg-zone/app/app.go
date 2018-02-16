package app

import (
	bam "github.com/cosmos/cosmos-sdk/baseapp"
	sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/cosmos/peggy/peg-zone/types"
)

type PeggyApp struct {
	*bam.BaseApp
	cdc *wire.Codec

	capKeyMainStore     *sdk.KVStoreKey
	capKeyWitnessStore  *sdk.KVStoreKey
	capKeyWithdrawStore *sdk.KVStoreKey

	accountMapper    sdk.AccountMapper
	witnessTxMapper  types.WitnessTxMapper
	withdrawTxMapper types.WithdrawTxMapper
}

func NewPeggy() *PeggyApp {
	app := &PeggyApp{
		BaseApp:             bam.NewBaseApp("Peggy"),
		cdc:                 MakeTxCodec(),
		capKeyMainStore:     sdk.NewKVStoreKey("main"),
		capKeyWitnessStore:  sdk.NewKVStoreKey("witness"),
		capKeyWithdrawStore: sdk.NewKVStoreKey("withdraw"),
	}

	app.accountMapper = auth.NewAccountMapperSealed(
		app.capKeyMainStore, // target store
		&types.AppAccount{}, // prototype
	)

	app.witnessTxMapper = types.NewWitnessTxMapper(app.capKeyWitnessStore)
	app.withdrawTxMapper = types.NewWithdrawTxMapper(app.capKeyWithdrawStore)

	// add handlers
	app.Router().AddRoute("bank", bank.NewHandler(bank.NewCoinKeeper(app.accountMapper)))

	bApp := bam.NewBaseApp(ppName)
	mountMultiStore(bApp, mainKey)
	err := bApp.LoadLatestVersion(mainKey)
	if err != nil {
		panic(err)
	}

	// register routes on new application
	accts := types.AccountMapper(mainKey)
	types.RegisterRoutes(bApp.Router(), accts)

	// set up ante and tx parsing
	setAnteHandler(bApp, accts)
	initBaseAppTxDecoder(bApp)

	return &PeggyApp{
		BaseApp: bApp,
		accts:   accts,
	}
}

func mountMultiStore(bApp *baseapp.BaseApp, keys ...*sdk.KVStoreKey) {
	// create substore for every key
	for _, key := range keys {
		bApp.MountStore(key, sdk.StoreTypeIAVL)
	}
}

func setAnteHandler(bApp *baseapp.BaseApp, accts sdk.AccountMapper) {
	// this checks auth, but may take fee is future, check for compatibility
	bApp.SetDefaultAnteHandler(
		auth.NewAnteHandler(accts))
}

func initBaseAppTxDecoder(bApp *baseapp.BaseApp) {
	cdc := types.MakeTxCodec()
	bApp.SetTxDecoder(func(txBytes []byte) (sdk.Tx, sdk.Error) {
		var tx = sdk.StdTx{}
		// StdTx.Msg is an interface whose concrete
		// types are registered in app/msgs.go.
		err := cdc.UnmarshalBinary(txBytes, &tx)
		if err != nil {
			return nil, sdk.ErrTxParse("").TraceCause(err, "")
		}
		return tx, nil
	})
}
