mod common;

use anyhow::Result;
use common::Context;

#[test]
fn it_async() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, assertEq } from "str";
            it("works", async () => {
                await null;
                console.error("async");
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> works ...
            async
            index.test.ts -> works PASSED
        "#,
    )?;
    Ok(())
}

#[test]
fn it_with_async_failure() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, assertEq } from "str";
            it("fails", async () => {
                await null;
                assertEq(true, false);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        r#"
            index.test.ts -> fails ...
            true
                !==
            false
            index.test.ts -> fails FAILED
        "#,
    )?;
    Ok(())
}

#[test]
fn before_each_async() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeEach } from "str";
            let variable;
            beforeEach(async () => {
                await null;
                variable = "set";
            });
            it("a", () => {
                console.error(variable);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            set
            index.test.ts -> a PASSED
        "#,
    )?;
    Ok(())
}

#[test]
fn after_each_async() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, afterEach } from "str";
            afterEach(async () => {
                await new Promise((resolve) => setTimeout(resolve, 10));
                console.error("async afterEach");
            });
            it("a", () => {});
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            async afterEach
            index.test.ts -> a PASSED
        "#,
    )?;
    Ok(())
}

#[test]
fn before_all_async() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeAll } from "str";
            let variable;
            beforeAll(async () => {
                await null;
                variable = "set";
            });
            it("a", () => {
                console.error(variable);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            set
            index.test.ts -> a PASSED
        "#,
    )?;
    Ok(())
}
