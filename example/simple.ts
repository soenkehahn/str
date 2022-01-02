import { it, assertEq } from "str";

it("works", () => {
  assertEq(true, true);
});

it("fails", () => {
  assertEq(true, false);
});
