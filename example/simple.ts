import { it, assertEq, describe } from "str";

describe("my app", () => {
  it("works", () => {
    assertEq(true, true);
  });

  it("fails", () => {
    assertEq(true, false);
  });
});
