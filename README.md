# `str` -- a simple typescript test-runner

**THIS PROJECT IS AN EXPERIMENT**

`str` aims to be a very simple test runner for test-suites written
in typescript.

## Installation from source

- Install [go](https://go.dev/)
- Clone the repo into `$STR_REPO`.
- `cd $STR_REPO`
- Build with `go build cmd/str.go`.
- Install with e.g. `cp str /usr/local/bin/`.

## How to use it?

- In your project, do:
  `yarn add --dev link:$STR_REPO/typescript-library`
- Create a test-suite, e.g.:

```typescript
import { it, assertEq, describe } from "str";

describe("my app", () => {
  it("works", () => {
    assertEq(true, true);
  });

  it("fails", () => {
    assertEq(true, false);
  });
});
```

- Run your test-suite with:
  `str $FILE_NAME`

## How to run the tests
- Install [rust](https://www.rust-lang.org/).
- Install [just](https://github.com/casey/just)
- Clone the repo into `$STR_REPO`.
- `cd $STR_REPO`
- Run all tests and checks with `just ci`.
- Run only the faster tests with `just test`.
