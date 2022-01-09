import { _strTestRunner } from ".";

export function beforeEach(f: () => void | Promise<void>): void {
  _strTestRunner.stackCurrent().aroundEachs.unshift((test) => async () => {
    await f();
    await test();
  });
}

export function afterEach(f: () => void | Promise<void>): void {
  _strTestRunner.stackCurrent().aroundEachs.push((test) => async () => {
    await test();
    await f();
  });
}

export function beforeAll(f: () => void): void {
  _strTestRunner.stackCurrent().beforeAlls.push(f);
}
