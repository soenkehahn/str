type StrTestRunner = {
  setTestFile: (file: string) => void;
  currentTestFile: string | null;
  fails: boolean;
  finalize: () => void;
};

export const _strTestRunner: StrTestRunner = {
  setTestFile: (file: string) => {
    _strTestRunner.currentTestFile = file;
  },
  currentTestFile: null,
  fails: false,
  finalize: () => {
    if (_strTestRunner.fails) {
      process.exit(1);
    }
  },
};

class StrTestFailure {}

export function it(testName: string, test: () => void) {
  console.error(`${_strTestRunner.currentTestFile} -> ${testName} ...`);
  try {
    test();
    console.error(`${_strTestRunner.currentTestFile} -> ${testName} PASSED`);
  } catch (exception) {
    if (exception instanceof StrTestFailure) {
      console.error(`${_strTestRunner.currentTestFile} -> ${testName} FAILED`);
      _strTestRunner.fails = true;
    } else {
      throw exception;
    }
  }
}

export function assertEq<T>(a: T, b: T) {
  if (a !== b) {
    console.error(`${a}\n    !==\n${b}`);
    throw new StrTestFailure();
  }
}
