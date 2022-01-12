import { exhaustivenessCheck } from "./utils";

export type LogKind = "start" | "passed" | "failed";

export function log(stack: Array<{ description: string }>, kind: LogKind) {
  const description = stack.map((x) => x.description).join(" -> ");
  let color;
  let kindSnippet;
  switch (kind) {
    case "start": {
      color = "\x1b[33m";
      kindSnippet = "...";
      break;
    }
    case "passed": {
      color = "\x1b[32m";
      kindSnippet = "PASSED";
      break;
    }
    case "failed": {
      color = "\x1b[31m";
      kindSnippet = "FAILED";
      break;
    }
    default: {
      exhaustivenessCheck(kind);
      break;
    }
  }
  console.error(`${color}${description} ${kindSnippet}\x1b[0m`);
}
