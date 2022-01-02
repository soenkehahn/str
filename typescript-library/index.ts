let has_failures = false;

export function it(x: string, test: () => void) {
  test();
}

export function assertEq<T>(a: T, b: T) {
  if (a !== b) {
    console.error(`${a}\n    !==\n${b}`);
    has_failures = true;
  }
}

export function finalize(): void {
  if (has_failures) {
    process.exit(1);
  }
}
