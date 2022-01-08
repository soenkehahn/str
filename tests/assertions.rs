mod common;

use anyhow::Result;
use common::Context;

#[test]
fn before_all_runs_before_all_tests_once() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeAll } from "str";

            let counter = 0;

            beforeAll(() => {
                counter += 1;
            });

            it("a", () => {
                assertEq(counter, 1);
            });

            it("b", () => {
                assertEq(counter, 1);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            index.test.ts -> a PASSED
            index.test.ts -> b ...
            index.test.ts -> b PASSED
        "#,
    );
    Ok(())
}

#[test]
fn before_all_allows_to_initialize_variables() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeAll } from "str";

            let test_variable;
            beforeAll(() => {
                test_variable = "set";
            });

            it("works", () => {
                assertEq(test_variable, "set");
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> works ...
            index.test.ts -> works PASSED
        "#,
    );
    Ok(())
}
