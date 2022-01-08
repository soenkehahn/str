mod common;

use anyhow::Result;
use common::Context;

#[test]
fn describe() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, describe } from "str";
            describe("description", () => {
                it("works", () => {
                    assertEq(true, true);
                });
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> description -> works ...
            index.test.ts -> description -> works PASSED
        "#,
    );
    Ok(())
}

#[test]
fn describe_bigger() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, describe } from "str";
            describe("description", () => {
                it("works", () => {
                    assertEq(true, true);
                });
                describe("second description", () => {
                    it("also works", () => {
                        assertEq(true, true);
                    });
                });
                it("works, too", () => {
                    assertEq(true, true);
                });
            });
            it("fails", () => {
                assertEq(true, false);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        r#"
            index.test.ts -> description -> works ...
            index.test.ts -> description -> works PASSED
            index.test.ts -> description -> second description -> also works ...
            index.test.ts -> description -> second description -> also works PASSED
            index.test.ts -> description -> works, too ...
            index.test.ts -> description -> works, too PASSED
            index.test.ts -> fails ...
            true
                !==
            false
            index.test.ts -> fails FAILED
        "#,
    );
    Ok(())
}

#[test]
fn describe_pops_description_stack_correctly_after_failures() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, describe } from "str";
            describe("description", () => {
                it("fails", () => {
                    assertEq(true, false);
                });
            });
            it("works", () => {
                assertEq(true, true);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        r#"
            index.test.ts -> description -> fails ...
            true
                !==
            false
            index.test.ts -> description -> fails FAILED
            index.test.ts -> works ...
            index.test.ts -> works PASSED
        "#,
    );
    Ok(())
}

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
