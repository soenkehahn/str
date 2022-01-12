mod common;

use anyhow::Result;
use common::Context;

#[test]
fn it_catches_exceptions() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {
                throw "foo";
            });
            it("b", () => {});
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        "
            index.test.ts -> a ...
            EXCEPTION: foo
            index.test.ts -> a FAILED
            index.test.ts -> b ...
            index.test.ts -> b PASSED
        ",
    )?;
    Ok(())
}

#[test]
fn it_catches_undefined_identifiers() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {
                notDefined();
            });
            it("b", () => {});
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        "
            index.test.ts -> a ...
            EXCEPTION: ReferenceError: notDefined is not defined
            index.test.ts -> a FAILED
            index.test.ts -> b ...
            index.test.ts -> b PASSED
        ",
    )?;
    Ok(())
}

#[test]
fn it_catches_async_exceptions() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {
                return new Promise((_, reject) => {
                    reject("foo");
                });
            });
            it("b", () => {});
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        "
            index.test.ts -> a ...
            EXCEPTION: foo
            index.test.ts -> a FAILED
            index.test.ts -> b ...
            index.test.ts -> b PASSED
        ",
    )?;
    Ok(())
}

#[test]
fn after_each_gets_run_when_it_throws() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, afterEach } from "str";
            afterEach(() => {
                console.error("afterEach");
            });
            it("a", () => {
                notDefined();
            });
            it("b", () => {});
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        "
            index.test.ts -> a ...
            EXCEPTION: ReferenceError: notDefined is not defined
            afterEach
            index.test.ts -> a FAILED
            index.test.ts -> b ...
            afterEach
            index.test.ts -> b PASSED
        ",
    )?;
    Ok(())
}
