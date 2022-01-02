# `str` -- a simple typescript test-runner

`str` aims to be a very simple test runner for test-suites written
in typescript.

## Installation from source

- Install [just](https://github.com/casey/just).
- Install [rust](https://www.rust-lang.org/).
- Clone the repo into `$STR_REPO`.
- Install with `cd $STR_REPO ; just install`.

## How to use it?

- In your project, do:
  `yarn add link:$STR_REPO`
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
