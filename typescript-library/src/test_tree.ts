import { log, logSummary } from "./logging";
import { exhaustivenessCheck } from "./utils";

export class StrTestFailure {}

export type StrTestRunner = {
  _stack: Array<TestTree>;
  _stackCurrent: () => TestTree;
  enterTestFile: (
    testFileName: string,
    dynamicImport: () => Promise<void>
  ) => Promise<void>;
  runTests: () => Promise<void>;
};

const newStrTestRunner = (): StrTestRunner => {
  let strTestRunner: StrTestRunner;
  strTestRunner = {
    _stack: [newTestTree()],
    _stackCurrent: () => strTestRunner._stack[strTestRunner._stack.length - 1],
    enterTestFile: async (
      testFileName: string,
      dynamicImport: () => Promise<void>
    ) => {
      let child: TestChild = {
        tag: "test file",
        tree: newTestTree(),
      };
      _strTestRunner._stackCurrent().children.push([testFileName, child]);
      _strTestRunner._stack.push(child.tree);
      await dynamicImport();
      _strTestRunner._stack.pop();
    },
    runTests: async () => {
      await runTestTree(strTestRunner._stack[0]);
    },
  };
  return strTestRunner;
};

type Test = () => void | Promise<void>;

type TestTree = {
  children: Array<[string, TestChild]>;
  beforeEachs: Array<() => void>;
  aroundEachs: Array<(test: Test) => () => Promise<void>>;
  beforeAlls: Array<() => void | Promise<void>>;
  afterAlls: Array<() => void | Promise<void>>;
};

export const newTestTree = (): TestTree => ({
  children: [],
  beforeEachs: [],
  aroundEachs: [],
  beforeAlls: [],
  afterAlls: [],
});

export type TestChild =
  | { tag: "it"; test: Test }
  | { tag: "describe"; tree: TestTree }
  | { tag: "test file"; tree: TestTree };

async function runTestTree(tree: TestTree) {
  const context: Context = {
    passes: 0,
    failures: 0,
    stack: [],
  };
  await runTestTreeHelper(context, tree);
  logSummary(context);
  if (context.failures > 0) {
    process.exit(1);
  }
}

export type Context = {
  passes: number;
  failures: number;
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
          let test = child.test;
          for (let i = context.stack.length - 1; i >= 0; i--) {
            const aroundEachs = context.stack[i].aroundEachs;
            for (const aroundEach of aroundEachs) {
              test = aroundEach(test);
            }
          }
          await test();
          context.passes++;
          log(context.stack, "passed");
        } catch (exception) {
          if (!(exception instanceof StrTestFailure)) {
            console.error(`EXCEPTION: ${exception}`);
          }
          context.failures++;
          log(context.stack, "failed");
        }
        break;
      }
      case "describe": {
        await runTestTreeHelper(context, child.tree);
        break;
      }
      case "test file": {
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
  for (const f of tree.afterAlls) {
    await f();
  }
}

export const _strTestRunner: StrTestRunner = newStrTestRunner();
