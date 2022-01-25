import {
  _strTestRunner,
  StrTestFailure,
  newTestTree,
  TestChild,
} from "./test_tree";

export function describe(description: string, inner: () => void): void {
  let child: TestChild = {
    tag: "describe",
    tree: newTestTree(),
  };
  _strTestRunner._stackCurrent().children.push([description, child]);

  _strTestRunner._stack.push(child.tree);
  inner();
  _strTestRunner._stack.pop();
}

export function it(testName: string, test: () => void | Promise<void>): void {
  _strTestRunner._stackCurrent().children.push([testName, { tag: "it", test }]);
}

export function assertEq<T>(a: T, b: T): void {
  if (a !== b) {
    console.error(`${a}\n    !==\n${b}`);
    throw new StrTestFailure();
  }
}
