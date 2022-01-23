mod common;

use anyhow::Result;
use common::Context;

#[test]
fn with_passing_tests() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "a.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {});
        "#,
    )?;
    context.write(
        "b.test.ts",
        r#"
            import { it } from "str";
            it("b", () => {});
        "#,
    )?;
    context.run_assert(
        "a.test.ts b.test.ts",
        0,
        "
            a.test.ts -> a ...
            a.test.ts -> a PASSED
            b.test.ts -> b ...
            b.test.ts -> b PASSED
        ",
    )?;
    Ok(())
}

#[test]
fn with_failures() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "a.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {});
        "#,
    )?;
    context.write(
        "b.test.ts",
        r#"
            import { it } from "str";
            it("failing", () => {
              throw "foo";
            });
            it("b", () => {});
        "#,
    )?;
    context.write(
        "c.test.ts",
        r#"
            import { it } from "str";
            it("c", () => {});
        "#,
    )?;
    context.run_assert(
        "a.test.ts b.test.ts c.test.ts",
        1,
        "
            a.test.ts -> a ...
            a.test.ts -> a PASSED
            b.test.ts -> failing ...
            EXCEPTION: foo
            b.test.ts -> failing FAILED
            b.test.ts -> b ...
            b.test.ts -> b PASSED
            c.test.ts -> c ...
            c.test.ts -> c PASSED
        ",
    )?;
    Ok(())
}
