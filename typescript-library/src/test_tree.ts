import { log } from "./logging";
import { exhaustivenessCheck } from "./utils";

export class StrTestFailure {}

export type StrTestRunner = {
  testFile: string | null;
  stack: Array<TestTree>;
  stackCurrent: () => TestTree;
  runTests: () => Promise<void>;
};

const newStrTestRunner = (): StrTestRunner => {
  let result: StrTestRunner;
  result = {
    testFile: null,
    stack: [newTestTree()],
    stackCurrent: () => result.stack[result.stack.length - 1],
    runTests: async () => {
      await runTestTree(result.testFile, result.stack[0]);
    },
  };
  return result;
};

type Test = () => void | Promise<void>;

type TestTree = {
  children: Array<[string, TestChild]>;
  beforeEachs: Array<() => void>;
  aroundEachs: Array<(test: Test) => () => Promise<void>>;
  beforeAlls: Array<() => void | Promise<void>>;
};

export const newTestTree = (): TestTree => ({
  children: [],
  beforeEachs: [],
  aroundEachs: [],
  beforeAlls: [],
});

export type TestChild =
  | { tag: "it"; test: Test }
  | { tag: "describe"; tree: TestTree };

async function runTestTree(fileName: string | null, tree: TestTree) {
  const context: Context = {
    fails: false,
    stack: fileName ? [{ description: fileName, aroundEachs: [] }] : [],
  };
  await runTestTreeHelper(context, tree);
  if (context.fails) {
    process.exit(1);
  }
}

type Context = {
  fails: boolean;
  stack: Array<{
    description: string;
    aroundEachs: Array<(test: Test) => () => Promise<void>>;
  }>;
};

async function runTestTreeHelper(
  context: Context,
  tree: TestTree
): Promise<void> {
  for (const f of tree.beforeAlls) {
    await f();
  }
  for (const [testName, child] of tree.children) {
    context.stack.push({
      description: testName,
      aroundEachs: tree.aroundEachs,
    });
    switch (child.tag) {
      case "it": {
        log(context.stack, "start");
        try {
          let test = async () => {
            try {
              return await child.test();
            } catch (exception) {
              if (!(exception instanceof StrTestFailure)) {
                console.error(`EXCEPTION: ${exception}`);
              }
              throw exception;
            }
          };
          for (let i = context.stack.length - 1; i >= 0; i--) {
            const aroundEachs = context.stack[i].aroundEachs;
            for (const aroundEach of aroundEachs) {
              test = aroundEach(test);
            }
          }
          await test();
          log(context.stack, "passed");
        } catch (exception) {
          log(context.stack, "failed");
          context.fails = true;
        }
        break;
      }
      case "describe": {
        await runTestTreeHelper(context, child.tree);
        break;
      }
      default: {
        exhaustivenessCheck(child);
        break;
      }
    }
    context.stack.pop();
  }
}

export const _strTestRunner: StrTestRunner = newStrTestRunner();
