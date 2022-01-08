export class StrTestFailure {}

type LogKind = "start" | "passed" | "failed";

export type StrTestRunner = {
  fails: boolean;
  finalize: () => void;

  testDescriptionStack: Array<string>;
  log: (kind: LogKind) => void;
};

export const _strTestRunner: StrTestRunner = {
  fails: false,
  finalize: () => {
    if (_strTestRunner.fails) {
      process.exit(1);
    }
  },

  testDescriptionStack: [],
  log: (kind: LogKind) => {
    const testDescription = _strTestRunner.testDescriptionStack.join(" -> ");
    let kindSnippet;
    if (kind === "start") {
      kindSnippet = "...";
    } else if (kind === "passed") {
      kindSnippet = "PASSED";
    } else if (kind === "failed") {
      kindSnippet = "FAILED";
    } else {
      exhaustivenessCheck(kind);
    }
    console.error(`${testDescription} ${kindSnippet}`);
  },
};

function exhaustivenessCheck(param: never) {}
