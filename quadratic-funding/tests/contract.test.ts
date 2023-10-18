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

  const result3 = await deployer.callContract({
    callee: contract,
    gasLimit: 20_000_000,
    funcName: "setProjectContributions",
    funcArgs: [
      e.Addr("erd1rf4hv70arudgzus0ymnnsnc4pml0jkywg2xjvzslg0mz4nn2tg7q7k0t6p"),
    ],
    value: 1_000_000_000n,
  });

  const result4 = await deployer.callContract({
    callee: contract,
    gasLimit: 20_000_000,
    funcName: "calculateRawMatch",
    funcArgs: [
      e.Addr("erd1rf4hv70arudgzus0ymnnsnc4pml0jkywg2xjvzslg0mz4nn2tg7q7k0t6p"),
    ],
  });

  const { returnData: matchingFund } = await world.query({
    callee: contract,
    funcName: "getMatchingFund",
  });

  const { returnData: totalRawMatch } = await world.query({
    callee: contract,
    funcName: "getTotalRawMatch",
  });

  const { returnData: getRawMatch } = await world.query({
    callee: contract,
    funcName: "getRawMatch",
    funcArgs: [
      e.Addr("erd1rf4hv70arudgzus0ymnnsnc4pml0jkywg2xjvzslg0mz4nn2tg7q7k0t6p"),
    ],
  });

  const { returnData: projectContributions } = await world.query({
    callee: contract,
    funcName: "getProjectContributions",
    funcArgs: [
      e.Addr("erd1rf4hv70arudgzus0ymnnsnc4pml0jkywg2xjvzslg0mz4nn2tg7q7k0t6p"),
    ],
  });

  console.log(
    "matchingFund, totalRawMatch",
    d.U64().topDecode(matchingFund[0]),
    d.U64().topDecode(totalRawMatch[0]),
    d.U64().topDecode(getRawMatch[0]),
    projectContributions,
    d
      .List(
        d.Tuple({
          contributor: d.Addr(),
          amount: d.U(),
        })
      )
      .topDecode(projectContributions[0])
  );
});
