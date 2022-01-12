import { _strTestRunner } from ".";

export function beforeEach(f: () => void | Promise<void>): void {
  _strTestRunner.stackCurrent().aroundEachs.unshift((test) => async () => {
    await f();
    await test();
  });
}

export function afterEach(f: () => void | Promise<void>): void {
  _strTestRunner.stackCurrent().aroundEachs.push((test) => async () => {
    try {
      await test();
    } finally {
      await f();
    }
  });
}

export function beforeAll(f: () => void | Promise<void>): void {
  _strTestRunner.stackCurrent().beforeAlls.push(f);
}
