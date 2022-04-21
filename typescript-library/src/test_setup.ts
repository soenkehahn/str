import { _strTestRunner } from ".";

export function beforeEach(f: () => void | Promise<void>): void {
  _strTestRunner._stackCurrent().aroundEachs.unshift((test) => async () => {
    await f();
    await test();
  });
}

export function afterEach(f: () => void | Promise<void>): void {
  _strTestRunner._stackCurrent().aroundEachs.push((test) => async () => {
    try {
      await test();
    } finally {
      await f();
    }
  });
}

export function beforeAll(f: () => void | Promise<void>): void {
  _strTestRunner._stackCurrent().beforeAlls.push(f);
}

export function afterAll(f: () => void | Promise<void>): void {
  _strTestRunner._stackCurrent().afterAlls.push(f);
}
