import { _strTestRunner, StrTestFailure } from "./test_runner";

export function describe(description: string, inner: () => void): void {
  _strTestRunner.testDescriptionStack.push(description);
  inner();
  _strTestRunner.testDescriptionStack.pop();
}

export function it(testName: string, test: () => void): void {
  _strTestRunner.testDescriptionStack.push(testName);
  _strTestRunner.log("start");
  try {
    test();
    _strTestRunner.log("passed");
  } catch (exception) {
    if (exception instanceof StrTestFailure) {
      _strTestRunner.log("failed");
      _strTestRunner.fails = true;
    } else {
      throw exception;
    }
  }
  _strTestRunner.testDescriptionStack.pop();
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
