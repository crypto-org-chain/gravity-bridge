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
  isOwner?: boolean;
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

  if (opts.isOwner) {
    await gravity.transferOwnership(signers[1].address);
  }

  if (opts.pause) {
    await gravity.connect(signers[1]).pause();
  }

  if (opts.unpause) {
    await gravity.connect(signers[1]).unpause();
  }
}

describe("pause tests", function () {
  it("non-owner cannot call pause()", async function () {
    await expect(runTest({
      pause: true
    })).to.be.revertedWith("Ownable: caller is not the owner");
  });

  it("non-owner cannot call unpause()", async function () {
    await expect(runTest({
      unpause: true
    })).to.be.revertedWith("Ownable: caller is not the owner");
  });

  it("owner can call pause() and unpause()", async function () {
    await runTest({
      isOwner: true,
      pause: true,
      unpause: true,
    });
  });
});
