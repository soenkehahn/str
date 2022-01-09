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

type LogKind = "start" | "passed" | "failed";

function log(testDescription: Array<string>, kind: LogKind) {
  const description = testDescription.join(" -> ");
  let kindSnippet;
  switch (kind) {
    case "start": {
      kindSnippet = "...";
      break;
    }
    case "passed": {
      kindSnippet = "PASSED";
      break;
    }
    case "failed": {
      kindSnippet = "FAILED";
      break;
    }
    default: {
      exhaustivenessCheck(kind);
      break;
    }
  }
  console.error(`${description} ${kindSnippet}`);
}

function exhaustivenessCheck(param: never) {}

type TestTree = {
  children: Array<[string, TestChild]>;
  beforeEachs: Array<() => void>;
  beforeAlls: Array<() => void>;
};

export const newTestTree = (): TestTree => ({
  children: [],
  beforeEachs: [],
  beforeAlls: [],
});

export type TestChild =
  | { tag: "it"; test: () => void }
  | { tag: "describe"; tree: TestTree };

function runTestTree(fileName: string | null, tree: TestTree) {
  const context = { stack: fileName ? [fileName] : [], fails: false };
  runTestTreeHelper(context, tree);
  if (context.fails) {
    process.exit(1);
  }
}

function runTestTreeHelper(
  context: { stack: Array<string>; fails: boolean },
  tree: TestTree
) {
  for (const f of tree.beforeAlls) {
    f();
  }
  for (const [testName, child] of tree.children) {
    context.stack.push(testName);
    switch (child.tag) {
      case "it": {
        log(context.stack, "start");
        for (const f of tree.beforeEachs) {
          f();
        }
        try {
          child.test();
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
