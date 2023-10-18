import { test, beforeEach, afterEach } from "vitest";
import { assertAccount, SWorld, SWallet, SContract, e, d } from "xsuite";

let world: SWorld;
let deployer: SWallet;
let contract: SContract;
let receiver: SWallet;

beforeEach(async () => {
  world = await SWorld.start();
  deployer = await world.createWallet({
    balance: 10_000_000_000n,
  });
  receiver = await world.createWallet();

  ({ contract } = await deployer.deployContract({
    code: "file:output/contract.wasm",
    codeMetadata: [],
    gasLimit: 10_000_000,
  }));
});

afterEach(async () => {
  await world.terminate();
});

test("Test", async () => {
  assertAccount(await contract.getAccountWithKvs(), {
    balance: 0n,
    allKvs: [],
  });

  const wallet = await world.createWallet({
    balance: 10_000_000_000n,
  });

  console.log("wallet", wallet);

  const result2 = await deployer.callContract({
    callee: contract,
    gasLimit: 20_000_000,
    funcName: "setMatchingFund",
    funcArgs: [e.U64(1 * 10 ** 18)],
  });

  const result1 = await deployer.callContract({
    callee: contract,
    gasLimit: 20_000_000,
    funcName: "setTotalRawMatch",
    funcArgs: [e.U64(1 * 10 ** 18)],
  });

  const { returnData: matchingFund } = await world.query({
    callee: contract,
    funcName: "getMatchingFund",
  });

  const {
    returnData: totalRawMatch,
    returnCode,
    returnMessage,
  } = await world.query({
    callee: contract,
    funcName: "getTotalRawMatch",
  });

  console.log(
    "matchingFund, totalRawMatch",
    d.U64().topDecode(matchingFund[0]),
    d.U64().topDecode(totalRawMatch[0])
  );
});
