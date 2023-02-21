package integration_tests

import (
	"context"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
)

// Validator out tests a validator that is not running the mandatory Ethereum node. This validator will be slashed and the bridge will remain functioning.

// Start the chain with validator
// 1. bring down an orchestrator
// 2. generate a batch
// 3. verify that the orchestrator has been jailed for missing signature
func (s *IntegrationTestSuite) TestValidatorOut() {
	s.Run("Shut down an orchestrator and verify the slashing is working", func() {
		// Shut down orchestrator 3
		s.T().Logf("Stop orchestrator 3")
		err := s.dockerPool.Purge(s.orchResources[3])
		s.Require().NoError(err, "error removing orchestrator 3")

		// Approve and check gravity contract
		s.T().Logf("approving Gravity to spend ERC 20")
		err = s.approveERC20()
		s.Require().NoError(err, "error approving spending balance for the gravity contract")
		val := s.chain.validators[0]
		allowance, err := s.getERC20AllowanceOf(common.HexToAddress(val.ethereumKey.address), gravityContract)
		s.Require().NoError(err, "error getting allowance of gravity contract spending on behalf of first validator")
		s.Require().Equal(UInt256Max(), allowance.BigInt(), "spending allowance not set correctly, got: %s", allowance.String())

		balance, err := s.getEthTokenBalanceOf(common.HexToAddress(val.ethereumKey.address), testERC20contract)
		s.Require().NoError(err, "error getting first validator balance")
		s.Require().Equal(sdk.NewUint(10000).BigInt(), balance.BigInt(), "balance was %s, expected 10000", balance.String())

		// Generate a transaction Eth -> Cosmos from validator 0 to itself
		s.T().Logf("sending to cosmos")
		valAddress, err := val.keyInfo.GetAddress()
		s.Require().NoError(err)
		err = s.sendToCosmos(valAddress, sdk.NewInt(200))
		s.Require().NoError(err, "error sending test denom to cosmos")
		kb, err := val.keyring()
		s.Require().NoError(err)
		clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", valAddress)
		s.Require().NoError(err)
		gbQueryClient := types.NewQueryClient(clientCtx)
		bankQueryClient := banktypes.NewQueryClient(clientCtx)
		var gravityDenom string
		s.Require().Eventuallyf(func() bool {
			res, err := bankQueryClient.AllBalances(context.Background(),
				&banktypes.QueryAllBalancesRequest{
					Address: valAddress.String(),
				})
			if err != nil {
				return false
			}
			denomRes, err := gbQueryClient.ERC20ToDenom(context.Background(),
				&types.ERC20ToDenomRequest{
					Erc20: testERC20contract.String(),
				})
			if err != nil {
				s.T().Logf("error querying ERC20 denom %s, %e", testERC20contract.String(), err)
				return false
			}
			s.Require().False(denomRes.CosmosOriginated, "ERC20-originated denom marked as cosmos originated")
			gravityDenom = denomRes.Denom

			for _, coin := range res.Balances {
				if coin.Denom == gravityDenom && coin.Amount.Equal(sdk.NewInt(200)) {
					return true
				}
			}

			s.T().Logf("balance not found, received %v", res.Balances)

			return false
		}, 105*time.Second, 10*time.Second, "balance never found on cosmos")

		// Generate a transaction Cosmos -> Eth from validator 0 to itself
		s.T().Logf("sending to ethereum")
		sendToEthereumMsg := types.NewMsgSendToEthereum(
			valAddress,
			s.chain.validators[0].ethereumKey.address,
			sdk.Coin{Denom: gravityDenom, Amount: sdk.NewInt(100)},
			sdk.Coin{Denom: gravityDenom, Amount: sdk.NewInt(1)},
		)
		response, err := s.chain.sendMsgs(*clientCtx, sendToEthereumMsg)
		s.Require().NoError(err)
		s.Require().Equal(uint32(0), response.Code)

		// Confirm batch has been generated and signed
		queryClient := types.NewQueryClient(clientCtx)
		s.Require().Eventuallyf(func() bool {
			res, err := queryClient.BatchTxConfirmations(context.Background(), &types.BatchTxConfirmationsRequest{BatchNonce: 1, TokenContract: testERC20contract.String()})
			if err != nil {
				s.T().Logf("error: %s", err)
			}
			if len(res.GetSignatures()) != 0 {
				return true
			}
			return false
		}, 5*time.Minute, 10*time.Second, "Can't find Batchtx signing info")

		// Check jail status of validators
		s.Require().Eventuallyf(func() bool {
			for i, validator := range s.chain.validators {
				keyring, err := validator.keyring()
				s.Require().NoError(err)
				valAddress, err := validator.keyInfo.GetAddress()
				s.Require().NoError(err)
				clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &keyring, "val", valAddress)
				s.Require().NoError(err)
				newQ := stakingtypes.NewQueryClient(clientCtx)
				valInfo, err := newQ.Validator(context.Background(), &stakingtypes.QueryValidatorRequest{ValidatorAddr: sdk.ValAddress(valAddress).String()})
				if err != nil {
					s.T().Logf("error: %s", err)
					return false
				}
				if i == 3 {
					s.Require().True(valInfo.GetValidator().IsJailed())
				} else {
					s.Require().False(valInfo.GetValidator().IsJailed())
				}
			}
			return true
		}, 5*time.Minute, 1*time.Minute, "can't find slashing info")
	})
}
