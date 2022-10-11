import chai from "chai";
import { ethers as hardhatether } from "hardhat";
import { ethers } from "ethers";
import { solidity } from "ethereum-waffle";

import { deployContracts } from "../test-utils";
import {
  examplePowers
} from "../test-utils/pure";

chai.use(solidity);
const { expect } = chai;

let gravityContract: ethers.Contract;
let erc20: ethers.Contract;
let signers: ethers.Signer[];

describe("migration tests", function () {

  beforeEach(async () => {
    // Prep and deploy contract
    // ========================
    signers = await hardhatether.getSigners();
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

    gravityContract = gravity
    erc20 = testERC20

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
  });

  it("non-owner cannot migrate token", async function () {
    // Try to migrate 1000 token to signe2 address
    await expect( gravityContract.connect(signers[1]).migrateToken(
        erc20.address,
        signers[2].getAddress(),
        1000,
        false)).to.be.revertedWith("Ownable: caller is not the owner")
    })

  it("non-owner cannot start migration", async function () {
    await expect( gravityContract.connect(signers[1]).startMigration())
        .to.be.revertedWith("Ownable: caller is not the owner")
  })

  it("owner cannot migrate token because migration has not started", async function () {
    // transfer ownership to signer1
    await gravityContract.transferOwnership(signers[1].getAddress());

    await expect( gravityContract.connect(signers[1]).migrateToken(
        erc20.address,
        signers[2].getAddress(),
        1000,
        false)).to.be.revertedWith("Migration has not started")
  })

  it("owner cannot migrate token because migration has stopped", async function () {
    // transfer ownership to signer1
    await gravityContract.transferOwnership(signers[1].getAddress());

    let migration = await gravityContract.functions.migration();
    expect(migration[0]).to.be.equal(false)

    await gravityContract.connect(signers[1]).startMigration()

    migration = await gravityContract.functions.migration()
    expect(migration[0]).to.be.equal(true)

    await gravityContract.connect(signers[1]).stopMigration()

    migration = await gravityContract.functions.migration();
    expect(migration[0]).to.be.equal(false)

    await expect( gravityContract.connect(signers[1]).migrateToken(
        erc20.address,
        signers[2].getAddress(),
        1000,
        false)).to.be.revertedWith("Migration has not started")
  })

  it("owner cannot migrate token because migration delay not elapsed", async function () {
    // transfer ownership to signer1
    await gravityContract.transferOwnership(signers[1].getAddress());

    let migration = await gravityContract.functions.migration();
    expect(migration[0]).to.be.equal(false)

    await gravityContract.connect(signers[1]).startMigration()

    migration = await gravityContract.functions.migration()
    expect(migration[0]).to.be.equal(true)

    await expect( gravityContract.connect(signers[1]).migrateToken(
        erc20.address,
        signers[2].getAddress(),
        1000,
        false)).to.be.revertedWith("Migration is not allowed yet")
  })

  /** DISABLE BECAUSE IT TAKES TOO LONG, UNCOMMENT TO ENABLE
  it("owner can migrate token", async function () {
    // transfer ownership to signer1
    await gravityContract.transferOwnership(signers[1].getAddress());

    let migration = await gravityContract.functions.migration();
    expect(migration[0]).to.be.equal(false)

    await gravityContract.connect(signers[1]).startMigration()

    migration = await gravityContract.functions.migration()
    expect(migration[0]).to.be.equal(true)

    // mine 21601 blocks, it will takes a lot of time but there is no better way?
    await hardhatether.provider.send("hardhat_mine", ["0x5460"])

    let signer2balance = (await erc20.functions.balanceOf(signers[2].getAddress()))[0];
    // Try to migrate 1000 token to signe2 address
    await gravityContract.connect(signers[1]).migrateToken(
        erc20.address,
        signers[2].getAddress(),
        1000,
        false);

    expect((await erc20.functions.balanceOf(gravityContract.address))[0]).to.equal(0);
    expect((await erc20.functions.balanceOf(signers[2].getAddress()))[0]).to.equal(signer2balance.add(1000));
  })
   **/

});
