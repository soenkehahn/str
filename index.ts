export function it(x: string, test: () => void) {
  test();
}

export function assertEq<T>(a: T, b: T) {
  if (a !== b) {
    console.error(`${a}\n    !==\n${b}`);
    process.exit(1);
  }
}

// export { it };
