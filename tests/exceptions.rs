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
            Ran 2 tests, 1 passed, 1 failed.
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
            Ran 2 tests, 1 passed, 1 failed.
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
            Ran 2 tests, 1 passed, 1 failed.
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
            afterEach
            EXCEPTION: ReferenceError: notDefined is not defined
            index.test.ts -> a FAILED
            index.test.ts -> b ...
            afterEach
            index.test.ts -> b PASSED
            Ran 2 tests, 1 passed, 1 failed.
        ",
    )?;
    Ok(())
}

#[test]
fn prints_exceptions_in_before_eachs() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeEach } from "str";
            beforeEach(() => {
                throw "test error";
            });
            it("a", () => {});
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        "
            index.test.ts -> a ...
            EXCEPTION: test error
            index.test.ts -> a FAILED
            Ran 1 test, 0 passed, 1 failed.
        ",
    )?;
    Ok(())
}
