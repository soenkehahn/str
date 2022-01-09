import { log } from "./logging";
import { exhaustivenessCheck } from "./utils";

export class StrTestFailure {}

export type StrTestRunner = {
  testFile: string | null;
  stack: Array<TestTree>;
  stackCurrent: () => TestTree;
  runTests: () => void;
};

const newStrTestRunner = (): StrTestRunner => {
  let result: StrTestRunner;
  result = {
    testFile: null,
    stack: [newTestTree()],
    stackCurrent: () => result.stack[result.stack.length - 1],
    runTests: () => {
      runTestTree(result.testFile, result.stack[0]);
    },
  };
  return result;
};

type TestTree = {
  children: Array<[string, TestChild]>;
  beforeEachs: Array<() => void>;
  aroundEachs: Array<(test: () => void) => () => void>;
  beforeAlls: Array<() => void>;
};

export const newTestTree = (): TestTree => ({
  children: [],
  beforeEachs: [],
  aroundEachs: [],
  beforeAlls: [],
});

export type TestChild =
  | { tag: "it"; test: () => void }
  | { tag: "describe"; tree: TestTree };

function runTestTree(fileName: string | null, tree: TestTree) {
  const context: Context = {
    fails: false,
    stack: fileName ? [{ description: fileName, aroundEachs: [] }] : [],
  };
  runTestTreeHelper(context, tree);
  if (context.fails) {
    process.exit(1);
  }
}

type Context = {
  fails: boolean;
  stack: Array<{
    description: string;
    aroundEachs: Array<(test: () => void) => () => void>;
  }>;
};

function runTestTreeHelper(context: Context, tree: TestTree) {
  for (const f of tree.beforeAlls) {
    f();
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
          test();
          log(context.stack, "passed");
        } catch (exception) {
          if (exception instanceof StrTestFailure) {
            log(context.stack, "failed");
            context.fails = true;
          } else {
            throw exception;
          }
        }
        break;
      }
      case "describe": {
        runTestTreeHelper(context, child.tree);
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
