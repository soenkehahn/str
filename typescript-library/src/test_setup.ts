import { _strTestRunner } from ".";

export function beforeAll(f: () => void): void {
  f();
}

export function beforeEach(f: () => void): void {
  _strTestRunner.stackCurrent().beforeEachs.push(f);
}
