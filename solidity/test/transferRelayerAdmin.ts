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
    isAdmin?: boolean;
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


    if (!opts.isAdmin) {
        await gravity.connect(signers[1]).transferAdmin(signers[1].address);
    }

}

describe("transferRelayerAdmin tests", function () {
    it("non admin cannot call transferAdmin", async function () {
        await expect(runTest({
            isAdmin: false
        })).to.be.revertedWith("AccessControl: account 0xead9c93b79ae7c1591b1fb5323bd777e86e150d4 is missing role 0xdf8b4c520ffe197c5343c6f5aec59570151ef9a492f2c624fd45ddde6135ec42");
    });

    it("admin can call transferAdmin", async function () {
        await expect(runTest({
            isAdmin: true
        }))
    });
});