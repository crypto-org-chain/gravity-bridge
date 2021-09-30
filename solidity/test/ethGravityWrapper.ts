import chai from "chai";
import { ethers } from "hardhat";
import { solidity } from "ethereum-waffle";
import { TestWETH } from "../typechain/TestWETH";
import { EthGravityWrapper } from "../typechain/EthGravityWrapper";

import { deployContracts } from "../test-utils";
import { examplePowers } from "../test-utils/pure";
import { Gravity } from "../typechain";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
chai.use(solidity);
const { expect } = chai;

describe("EthGravityWrapper tests", function () {
  let testWETH: TestWETH;
  let gravity: Gravity;
  let ethGravityWrapper: EthGravityWrapper;
  let signers: SignerWithAddress[];

  beforeEach(async () => {
    // Prep and deploy WETH
    const TestWETH = await ethers.getContractFactory("TestWETH");
    testWETH = (await TestWETH.deploy()) as TestWETH;

    // Prep and deploy contract Gravity
    signers = await ethers.getSigners();
    const gravityId = ethers.utils.formatBytes32String("foo");
    // This is the power distribution on the Cosmos hub as of 7/14/2020
    const powers = examplePowers();
    const validators = signers.slice(0, powers.length);
    const powerThreshold = 6666;
    ({ gravity } = await deployContracts(
      gravityId,
      validators,
      powers,
      powerThreshold
    ));

    // Prep and deploy contract EthGravityWrapper
    const EthGravityWrapper = await ethers.getContractFactory(
      "EthGravityWrapper"
    );
    ethGravityWrapper = (await EthGravityWrapper.deploy(
      testWETH.address,
      gravity.address
    )) as EthGravityWrapper;
  });

  it("allows eth to be sent", async function () {
    const destination = await signers[1].getAddress();

    // Check balance before on Gravity.sol
    expect((await testWETH.functions.balanceOf(gravity.address))[0]).to.equal(
      0
    );

    // Sending ETH over
    await testWETH.functions.approve(gravity.address, 100);
    await expect(
      ethGravityWrapper.functions.sendToCronosEth(destination, {
        value: "100",
      })
    )
      .to.emit(ethGravityWrapper, "sendToCronosEthEvent")
      .withArgs(await signers[0].getAddress, destination, 100);

    // Check balance after on Gravity.sol
    expect(
      (await testWETH.functions.balanceOf(ethGravityWrapper.address))[0]
    ).to.equal(0);
    expect((await testWETH.functions.balanceOf(gravity.address))[0]).to.equal(
      100
    );
  });

  it("does not reduce WETH allowance for gravity", async function () {
    const destination = await signers[1].getAddress();

    const allowancePrior = await testWETH.allowance(
      ethGravityWrapper.address,
      gravity.address
    );

    // Sending ETH over
    await testWETH.functions.approve(gravity.address, 10000000);
    await ethGravityWrapper.functions.sendToCronosEth(destination, {
      value: "10000000",
    });

    const allowanceAfter = await testWETH.allowance(
      ethGravityWrapper.address,
      gravity.address
    );
    expect(allowancePrior.toString()).to.equals(allowanceAfter.toString());
  });

  it("checks for eth amount > 0", async function () {
    const destination = await signers[1].getAddress();

    // Check balance before on Gravity.sol
    expect((await testWETH.functions.balanceOf(gravity.address))[0]).to.equal(
      0
    );

    // Sending ETH over with 0 value
    await testWETH.functions.approve(gravity.address, 100);
    await expect(
      ethGravityWrapper.functions.sendToCronosEth(destination)
    ).to.be.revertedWith("Amount should be greater than 0");

    // Check balance after on Gravity.sol
    expect((await testWETH.functions.balanceOf(gravity.address))[0]).to.equal(
      0
    );
  });
});
