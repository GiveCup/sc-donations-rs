import { test, beforeEach, afterEach } from "vitest";
import { assertAccount, SWorld, SWallet, SContract, e } from "xsuite";

let world: SWorld;
let deployer: SWallet;
let contract: SContract;
let wallet: SWallet;
let receiver: SWallet;

beforeEach(async () => {
  world = await SWorld.start();
  deployer = await world.createWallet({ balance: 10_000_000_000n });
  receiver = await world.createWallet();

  ({ contract } = await deployer.deployContract({
    code: "file:output/contract.wasm",
    codeMetadata: [],
    gasLimit: 10_000_000,
  }));

  console.log("contract", contract);
});

afterEach(async () => {
  await world.terminate();
});

test("Test", async () => {
  console.log("deployer", deployer);
  const wallet = await world.createWallet({
    balance: 10_000_000_000n,
  });

  console.log("wallet", wallet);

  assertAccount(await contract.getAccountWithKvs(), {
    balance: 0n,
    allKvs: [],
  });

  const result2 = await deployer.callContract({
    callee: contract,
    gasLimit: 20_000_000,
    funcName: "addOrganization",
    funcArgs: [e.Addr(receiver.signer.toString())],
  });

  const result1 = await wallet.callContract({
    callee: contract,
    gasLimit: 20_000_000,
    funcName: "donate",
    funcArgs: [e.Usize(1)],
    value: BigInt(1_000),
  });

  assertAccount(await receiver.getAccountWithKvs(), {
    balance: 1_000n,
  });

  const { returnData, returnCode, returnMessage } = await world.query({
    callee: contract,
    funcName: "getOrganizations",
  });

  console.log("returnData", returnData);

  // const result1 = await wallet.callContract({
  //   callee: contract,
  //   gasLimit: 20_000_000,
  //   funcName: "donate",
  //   funcArgs: [],
  //   value: BigInt(1_000),
  // });
});
