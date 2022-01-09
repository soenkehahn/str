import { _strTestRunner } from ".";

export function beforeEach(f: () => void): void {
  _strTestRunner.stackCurrent().aroundEachs.unshift((test) => () => {
    f();
    test();
  });
}

export function afterEach(f: () => void): void {
  _strTestRunner.stackCurrent().aroundEachs.push((test) => () => {
    test();
    f();
  });
}

export function beforeAll(f: () => void): void {
  _strTestRunner.stackCurrent().beforeAlls.push(f);
}
