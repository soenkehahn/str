import { _strTestRunner, StrTestFailure, newTestTree } from "./test_runner";

export function describe(description: string, inner: () => void): void {
  let child = newTestTree();
  _strTestRunner.stackCurrent().children.push([
    description,
    {
      tag: "describe",
      tree: child,
    },
  ]);

  _strTestRunner.stack.push(child);
  inner();
  _strTestRunner.stack.pop();
}

export function it(testName: string, test: () => void): void {
  _strTestRunner.stackCurrent().children.push([testName, { tag: "it", test }]);
}

export function assertEq<T>(a: T, b: T): void {
  if (a !== b) {
    console.error(`${a}\n    !==\n${b}`);
    throw new StrTestFailure();
  }
}
