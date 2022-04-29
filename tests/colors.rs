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
            format!("Ran 2 tests, {}, {}.", "1 passed".green(), "1 failed".red()).normal(),
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )?;
    Ok(())
}

#[test]
fn all_pass() -> Result<()> {
    colored::control::set_override(true);
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {});
            it("b", () => {});
        "#,
    )?;
    context.run_assert_with_colors(
        "index.test.ts",
        0,
        &vec![
            "index.test.ts -> a ...".yellow(),
            "index.test.ts -> a PASSED".green(),
            "index.test.ts -> b ...".yellow(),
            "index.test.ts -> b PASSED".green(),
            format!("Ran 2 tests, {}, 0 failed.", "2 passed".green()).normal(),
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )?;
    Ok(())
}

#[test]
fn all_fail() -> Result<()> {
    colored::control::set_override(true);
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {
                throw "foo";
            });
            it("b", () => {
                throw "foo";
            });
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
            "EXCEPTION: foo".normal(),
            "index.test.ts -> b FAILED".red(),
            format!("Ran 2 tests, {}, {}.", "0 passed".green(), "2 failed".red()).normal(),
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )?;
    Ok(())
}

#[test]
fn three_tests() -> Result<()> {
    colored::control::set_override(true);
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {});
            it("b", () => {});
            it("c", () => {});
        "#,
    )?;
    context.run_assert_with_colors(
        "index.test.ts",
        0,
        &vec![
            "index.test.ts -> a ...".yellow(),
            "index.test.ts -> a PASSED".green(),
            "index.test.ts -> b ...".yellow(),
            "index.test.ts -> b PASSED".green(),
            "index.test.ts -> c ...".yellow(),
            "index.test.ts -> c PASSED".green(),
            format!("Ran 3 tests, {}, 0 failed.", "3 passed".green()).normal(),
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )?;
    Ok(())
}

#[test]
fn only_one_test() -> Result<()> {
    colored::control::set_override(true);
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("a", () => {});
        "#,
    )?;
    context.run_assert_with_colors(
        "index.test.ts",
        0,
        &vec![
            "index.test.ts -> a ...".yellow(),
            "index.test.ts -> a PASSED".green(),
            format!("Ran 1 test, {}, 0 failed.", "1 passed".green()).normal(),
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )?;
    Ok(())
}

#[test]
fn ignored_tests() -> Result<()> {
    colored::control::set_override(true);
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { xit } from "str";
            xit("a", () => {});
        "#,
    )?;
    context.run_assert_with_colors(
        "index.test.ts",
        0,
        &vec![
            "index.test.ts -> a IGNORED".yellow(),
            format!("Ran 0 tests, {}, 0 failed, 1 ignored.", "0 passed".green()).normal(),
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )?;
    Ok(())
}
