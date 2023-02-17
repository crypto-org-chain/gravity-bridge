package integration_tests

// package imports
import (
	"context"
	"fmt"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
)

// We have 4 validators running so this totals to 100 tx's
const transactionsPerValidator int64 = 25
const cosmosSentAmt int64 = 100
const ethSentAmt int64 = 1

func (s *IntegrationTestSuite) TestTransactionStress() {
	s.Run("Transaction stress test", func() {
		fmt.Println("StressTestTransaction starting")

		// Approve spend & verify funds on ethereum
		for _, validator := range s.chain.validators {
			err := s.SendEthTransaction(&validator.ethereumKey, testERC20contract, PackApproveERC20(gravityContract))
			s.Require().NoError(err, "error approving spend")

			balance, err := s.getEthTokenBalanceOf(common.HexToAddress(validator.ethereumKey.address), testERC20contract)
			s.Require().NoError(err, "error getting balance")
			s.Require().Equal(sdk.NewUint(10000).BigInt(), balance.BigInt(), "balance was %s, expected 10000", balance.String())
		}

		sendAmt := sdk.NewInt(cosmosSentAmt)
		// Send many tx's Ethereum -> Cosmos on ethereum
		for i, validator := range s.chain.validators {
			s.T().Logf("sending %d tx's to cosmos for validator %d ..", transactionsPerValidator, i+1)
			valAddr, err := validator.keyInfo.GetAddress()
			s.Require().NoError(err)
			for j := 0; j < int(transactionsPerValidator); j++ {
				s.Require().NoError(s.SendEthTransaction(&validator.ethereumKey, gravityContract, PackSendToCosmos(testERC20contract, valAddr, sendAmt)))
			}

			s.T().Logf("%d Tx sent.", transactionsPerValidator)
		}

		var gravityDenom string
		for i, validator := range s.chain.validators {
			kb, err := validator.keyring()
			s.Require().NoError(err)
			valAddr, err := validator.keyInfo.GetAddress()
			s.Require().NoError(err)
			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", valAddr)
			s.Require().NoError(err)
			bankQueryClient := banktypes.NewQueryClient(clientCtx)

			// Checking balance correctly received
			s.Require().Eventuallyf(func() bool {
				s.T().Logf("Checking validator %d", i+1)
				res, err := bankQueryClient.AllBalances(context.Background(),
					&banktypes.QueryAllBalancesRequest{
						Address: valAddr.String(),
					})
				if err != nil {
					s.T().Logf("error: %s", err)
					return false
				}

				gbQueryClient := types.NewQueryClient(clientCtx)
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
					if coin.Denom == gravityDenom && coin.Amount.Equal(sdk.NewInt(cosmosSentAmt*transactionsPerValidator)) {
						s.T().Logf("Expected funds received for validator %d, balance: %v", i+1, coin)
						return true
					}
				}

				s.T().Logf("balance not found, received %v", res.Balances)

				return false
			}, 105*time.Second, 10*time.Second, "balance never found on cosmos")
		}
		fmt.Println("Ethereum -> Cosmos stress test completed.")

		for i, validator := range s.chain.validators {
			s.T().Logf("sending %d tx's to ethereum for validator %d ..", transactionsPerValidator, i+1)

			keyring, err := validator.keyring()
			s.Require().NoError(err)
			valAddr, err := validator.keyInfo.GetAddress()
			s.Require().NoError(err)
			sendToEthereumMsg := types.NewMsgSendToEthereum(
				valAddr,
				validator.ethereumKey.address,
				sdk.Coin{Denom: gravityDenom, Amount: sdk.NewInt(ethSentAmt)},
				sdk.Coin{Denom: gravityDenom, Amount: sdk.NewInt(0)},
			)

			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &keyring, "val", valAddr)
			s.Require().NoError(err)

			for j := 0; j < int(transactionsPerValidator); j++ {
				s.Require().Eventuallyf(func() bool {
					response, err := s.chain.sendMsgs(*clientCtx, sendToEthereumMsg)
					s.Require().NoError(err)
					if uint32(0) == response.Code {
						return true
					} else {
						s.T().Logf("wait for previous transaction to be executed")
						return false
					}
				}, time.Second*180, time.Second*2, "balance never found")

			}
			s.T().Logf("%d Tx sent.", transactionsPerValidator)
		}

		for i, validator := range s.chain.validators {
			s.Require().Eventuallyf(func() bool {
				s.T().Logf("Checking validator %d", i+1)

				balance, err := s.getEthTokenBalanceOf(common.HexToAddress(validator.ethereumKey.address), testERC20contract)
				s.Require().NoError(err, "error getting destination balance")

				if !balance.Equal(sdk.NewInt(10000 - (cosmosSentAmt * transactionsPerValidator) + (ethSentAmt * transactionsPerValidator))) {
					s.T().Logf("funds not received yet, dest balance: %s", balance.String())
					return false
				}

				s.T().Logf("Funds received for validator %d, current balance: %v", i+1, balance.String())
				return true
			}, time.Second*180, time.Second*10, "balance never found")
		}
		fmt.Println("Cosmos -> Ethereum stress test completed.")

		fmt.Println("StressTestTransaction completed successfully")
	})
}
