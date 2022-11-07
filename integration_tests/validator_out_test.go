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

// Start the chain with validators
func (s *IntegrationTestSuite) TestValidatorOut() {
	s.Run("Bring up chain, and test the valset update", func() {
		s.T().Logf("Stop orchestrator 3")
		err := s.dockerPool.Purge(s.orchResources[3])
		s.Require().NoError(err, "error removing orchestrator 3")

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

		// Send from validator 0 on eth to itself on cosmos
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

		s.T().Logf("sending to ethereum")
		sendToEthereumMsg := types.NewMsgSendToEthereum(
			valAddress,
			s.chain.validators[0].ethereumKey.address,
			sdk.Coin{Denom: gravityDenom, Amount: sdk.NewInt(100)},
			sdk.Coin{Denom: gravityDenom, Amount: sdk.NewInt(1)},
		)
		// Send NewMsgSendToEthereum Message
		response, err := s.chain.sendMsgs(*clientCtx, sendToEthereumMsg)
		if err != nil {
			s.T().Logf("error: %s", err)
		}
		if response.Code != 0 {
			if response.Code != 32 {
				s.T().Log(response)
			}
		}

		// Create transaction batch trigger by validator 2
		val2Address, err := s.chain.validators[2].keyInfo.GetAddress()
		s.Require().NoError(err)
		batchTx := types.NewMsgRequestBatchTx(gravityDenom, val2Address)
		keyRing2, err := s.chain.validators[2].keyring()
		s.Require().NoError(err)
		s.Require().Eventuallyf(func() bool {
			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &keyRing2, "val", val2Address)
			s.Require().NoError(err)
			response, err := s.chain.sendMsgs(*clientCtx, batchTx)
			s.T().Logf("batch response: %s", response)
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}

			if response.Code != 0 {
				if response.Code != 32 {
					s.T().Log(response)
				}
				return false
			}
			return true
		}, 5*time.Minute, 1*time.Second, "can't create TX batch successfully")

		// Confirm batchtx signatures by validator 2
		queryClient := types.NewQueryClient(clientCtx)
		s.Require().Eventuallyf(func() bool {
			res, err := queryClient.BatchTxConfirmations(context.Background(), &types.BatchTxConfirmationsRequest{BatchNonce: 1, TokenContract: testERC20contract.String()})
			s.Require().NoError(err)
			s.Require().NotEmpty(res.GetSignatures())
			return true
		}, 5*time.Minute, 1*time.Minute, "Can't find Batchtx signing info")

		// Check jail status of validators
		s.Require().Eventuallyf(func() bool {
			orch3Key := s.chain.validators[3]
			keyring3, err := orch3Key.keyring()
			s.Require().NoError(err)
			val3Address, err := s.chain.validators[3].keyInfo.GetAddress()
			s.Require().NoError(err)
			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &keyring3, "val", val3Address)
			s.Require().NoError(err)
			newQ := stakingtypes.NewQueryClient(clientCtx)
			valThree, err := newQ.Validator(context.Background(), &stakingtypes.QueryValidatorRequest{ValidatorAddr: sdk.ValAddress(val3Address).String()})
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}
			s.Require().True(valThree.GetValidator().IsJailed())

			val2Address, err := s.chain.validators[2].keyInfo.GetAddress()
			s.Require().NoError(err)
			valTwo, err := newQ.Validator(context.Background(), &stakingtypes.QueryValidatorRequest{ValidatorAddr: sdk.ValAddress(val2Address).String()})
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}
			s.Require().False(valTwo.GetValidator().IsJailed())

			val1Address, err := s.chain.validators[1].keyInfo.GetAddress()
			s.Require().NoError(err)
			valOne, err := newQ.Validator(context.Background(), &stakingtypes.QueryValidatorRequest{ValidatorAddr: sdk.ValAddress(val1Address).String()})
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}
			s.Require().False(valOne.GetValidator().IsJailed())

			val0Address, err := s.chain.validators[0].keyInfo.GetAddress()
			s.Require().NoError(err)
			valZero, err := newQ.Validator(context.Background(), &stakingtypes.QueryValidatorRequest{ValidatorAddr: sdk.ValAddress(val0Address).String()})
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}
			s.Require().False(valZero.GetValidator().IsJailed())
			return true
		}, 5*time.Minute, 1*time.Minute, "can't find slashing info")
	})
}
