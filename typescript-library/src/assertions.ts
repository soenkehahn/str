import { _strTestRunner, StrTestFailure } from "./test_runner";

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

export function assertEq<T>(a: T, b: T): void {
  if (a !== b) {
    console.error(`${a}\n    !==\n${b}`);
    throw new StrTestFailure();
  }
}

export function beforeAll(f: () => void): void {
  f();
}
