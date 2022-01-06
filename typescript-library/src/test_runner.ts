export class StrTestFailure {}

export type StrTestRunner = {
  setTestFile: (file: string) => void;
  currentTestFile: string | null;
  fails: boolean;
  finalize: () => void;
};

export const _strTestRunner: StrTestRunner = {
  setTestFile: (file: string) => {
    _strTestRunner.currentTestFile = file;
  },
  currentTestFile: null,
  fails: false,
  finalize: () => {
    if (_strTestRunner.fails) {
      process.exit(1);
    }
  },
};
