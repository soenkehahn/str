mod common;

use anyhow::Result;
use colored::Colorize;
use common::Context;

#[test]
fn colorizes_output() -> Result<()> {
    colored::control::set_override(true);
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
    context.run_assert_with_colors(
        "index.test.ts",
        1,
        &vec![
            "index.test.ts -> a ...".yellow(),
            "EXCEPTION: foo".normal(),
            "index.test.ts -> a FAILED".red(),
            "index.test.ts -> b ...".yellow(),
            "index.test.ts -> b PASSED".green(),
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )?;
    Ok(())
}
