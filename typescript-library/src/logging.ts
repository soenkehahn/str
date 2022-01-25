import { exhaustivenessCheck } from "./utils";
import { Context } from "./test_tree";

export type LogKind = "start" | "passed" | "failed";

export function log(stack: Array<{ description: string }>, kind: LogKind) {
  const description = stack.map((x) => x.description).join(" -> ");
  let kindSnippet;
  let color = (s: string) => s;
  switch (kind) {
    case "start": {
      color = yellow;
      kindSnippet = "...";
      break;
    }
    case "passed": {
      color = green;
      kindSnippet = "PASSED";
      break;
    }
    case "failed": {
      color = red;
      kindSnippet = "FAILED";
      break;
    }
    default: {
      exhaustivenessCheck(kind);
      break;
    }
  }
  console.error(color(`${description} ${kindSnippet}`));
}

export function logSummary(context: Context) {
  const numberOfTests = context.passes + context.failures;
  const noun = numberOfTests == 1 ? "test" : "tests";
  let message = `Ran ${numberOfTests} ${noun}, `;
  message += green(`${context.passes} passed`);
  message += `, `;
  let failures = `${context.failures} failed`;
  if (context.failures > 0) {
    failures = red(failures);
  }
  message += failures;
  message += `.`;
  console.error(message);
}

const green = (s: string): string => `\x1b[32m${s}\x1b[0m`;
const red = (s: string): string => `\x1b[31m${s}\x1b[0m`;
const yellow = (s: string): string => `\x1b[33m${s}\x1b[0m`;
