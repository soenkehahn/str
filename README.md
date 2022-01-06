# `str` -- a simple typescript test-runner

`str` aims to be a very simple test runner for test-suites written
in typescript.

## Installation from source

- Install [rust](https://www.rust-lang.org/).
- Clone the repo into `$STR_REPO`.
- Install with `cd $STR_REPO ; cargo install --path .`.

## How to use it?

- In your project, do:
  `yarn add --dev link:$STR_REPO/typescript-library`
- Create a test-suite, e.g.:

```typescript
import { it, assertEq } from "str";

it("works", () => {
  assertEq(true, true);
});

it("fails", () => {
  assertEq(true, false);
});
```

- Run your test-suite with:
  `str $FILE_NAME`
