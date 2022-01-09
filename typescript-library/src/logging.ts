import { exhaustivenessCheck } from "./utils";

export type LogKind = "start" | "passed" | "failed";

export function log(stack: Array<{ description: string }>, kind: LogKind) {
  const description = stack.map((x) => x.description).join(" -> ");
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
