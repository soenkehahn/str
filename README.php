# `str` -- a simple typescript test-runner

`str` aims to be a very simple test runner for test-suites written
in typescript.

## How to use it?

- Clone the repo into `$STR_REPO`.
- Install with `cd $STR_REPO ; just install`.
- In your project, do:
  `yarn add link:$STR_REPO`
- Create a test-suite, e.g.:

```typescript
<?php include("example/simple.ts"); ?>
```

- Run your test-suite with:
  `$ str $FILE_NAME`
