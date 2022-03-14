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

  // Send some token to the contract
  // =====================================
  await testERC20.functions.approve(gravity.address, 1000);
  await expect(gravity.functions.sendToCronos(
      testERC20.address,
      "0xffffffffffffffffffffffffffffffffffffffff",
      1000
  )).to.emit(gravity, 'SendToCosmosEvent').withArgs(
      testERC20.address,
      await signers[0].getAddress(),
      "0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff",
      1000,
      2
  );

  expect((await testERC20.functions.balanceOf(gravity.address))[0]).to.equal(1000);
  expect((await gravity.functions.state_lastEventNonce())[0]).to.equal(2);


  if (opts.isOwner) {
    await gravity.transferOwnership(signers[1].address);
  }

  let signer2balance = (await testERC20.functions.balanceOf(signers[2].address))[0];

  // Try to migrate 1000 token to signe2 address
  await gravity.connect(signers[1]).migrateToken(
      testERC20.address,
      signers[2].address,
      1000);

  expect((await testERC20.functions.balanceOf(gravity.address))[0]).to.equal(0);
  expect((await testERC20.functions.balanceOf(signers[2].address))[0]).to.equal(signer2balance.add(1000));
}

describe("migration tests", function () {
  it("non-owner cannot migrate token", async function () {
    await expect(runTest({
    })).to.be.revertedWith("Ownable: caller is not the owner");
  });

  it("owner can migrate token", async function () {
    await runTest({
      isOwner: true,
    });
  });
});
