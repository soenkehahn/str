mod common;

use anyhow::Result;
use common::Context;

#[test]
fn it() -> Result<()> {
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
    );
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
    );
    Ok(())
}
