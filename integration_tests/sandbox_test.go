package integration_tests

import "fmt"

func (s *IntegrationTestSuite) TestSandbox() {
	s.Run("Bring up chain and see what happens", func() {
		// Write whatever code you need here. You can refer to happy_path_test.go
		// Alternatively, wait for the chain to start and use this validator's
		// private key to perform ethereum / cosmos operations.
		//
		// The JSON-RPC server is hosted at 0.0.0.0:8545
		fmt.Printf("Validator 0 information: %+v", s.chain.validators[0].ethereumKey)

		s.T().Log("Chain will now keep running. Press enter to exit at any time")

		fmt.Scanln()

	})
}
