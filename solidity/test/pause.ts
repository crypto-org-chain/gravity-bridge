import chai from "chai";
import { ethers } from "hardhat";
import { solidity } from "ethereum-waffle";

import { deployContracts } from "../test-utils";
import {
  getSignerAddresses,
  makeCheckpoint,
  signHash,
  makeTxBatchHash,
  examplePowers
} from "../test-utils/pure";

chai.use(solidity);
const { expect } = chai;


async function runTest(opts: {
  isRelayer?: boolean;
  isControl?: boolean;
  pause?: boolean;
  unpause?: boolean;
}) {


  // Prep and deploy contract
  // ========================
  const signers = await ethers.getSigners();
  const gravityId = ethers.utils.formatBytes32String("foo");
  // This is the power distribution on the Cosmos hub as of 7/14/2020
  let powers = examplePowers();
  let validators = signers.slice(0, powers.length);
  const powerThreshold = 6666;
  const {
    gravity,
    testERC20,
    checkpoint: deployCheckpoint
  } = await deployContracts(gravityId, validators, powers, powerThreshold);

  if (opts.isControl) {
    await gravity.grantRole(
        await gravity.CONTROL(),
        signers[1].address,
    );
  }

  if (opts.pause) {
    await gravity.connect(signers[1]).pause();
  }

  if (opts.unpause) {
    await gravity.connect(signers[1]).unpause();
  }
}

describe("pause tests", function () {
  it("non control admin cannot call pause()", async function () {
    await expect(runTest({
      pause: true
    })).to.be.revertedWith("AccessControl: account 0xead9c93b79ae7c1591b1fb5323bd777e86e150d4 is missing role 0xbdded29a54e6a5d6169bedea55373b06f57e35d7b0f67ac187565b435e2cc943");
  });

  it("non control admin call unpause()", async function () {
    await expect(runTest({
      unpause: true
    })).to.be.revertedWith("AccessControl: account 0xead9c93b79ae7c1591b1fb5323bd777e86e150d4 is missing role 0xbdded29a54e6a5d6169bedea55373b06f57e35d7b0f67ac187565b435e2cc943");
  });

  it("control admin can call pause() and unpause()", async function () {
    await runTest({
      isControl: true,
      pause: true,
      unpause: true,
    });
  });
});
