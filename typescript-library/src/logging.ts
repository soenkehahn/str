import { exhaustivenessCheck } from "./utils";

export type LogKind = "start" | "passed" | "failed";

export function log(testDescription: Array<string>, kind: LogKind) {
  const description = testDescription.join(" -> ");
  let kindSnippet;
  switch (kind) {
    case "start": {
      kindSnippet = "...";
      break;
    }
    case "passed": {
      kindSnippet = "PASSED";
      break;
    }
    case "failed": {
      kindSnippet = "FAILED";
      break;
    }
    default: {
      exhaustivenessCheck(kind);
      break;
    }
  }
  console.error(`${description} ${kindSnippet}`);
}
