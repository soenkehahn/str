mod common;

use anyhow::Result;
use common::assert_contains;
use common::Context;
use cradle::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn simple_test_failure() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";
            it("fails", () => {
                assertEq(true, false);
            });
        "#,
    )?;
    context.run_assert(
        "src/index.test.ts",
        1,
        "
            src/index.test.ts -> fails ...
            true
                !==
            false
            src/index.test.ts -> fails FAILED
        ",
    );
    Ok(())
}

#[test]
fn simple_test_success() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";
            it("works", () => {
                assertEq(true, true);
            });
        "#,
    )?;
    context.run_assert(
        "src/index.test.ts",
        0,
        "
            src/index.test.ts -> works ...
            src/index.test.ts -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn typescript_gets_compiled_to_javascript() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";
            it("works", () => {
                const x: boolean = true;
                assertEq(true, x);
            });
        "#,
    )?;
    context.run_assert(
        "src/index.test.ts",
        0,
        "
            src/index.test.ts -> works ...
            src/index.test.ts -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn multiple_tests_passing() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";

            it("works", () => {
                assertEq(true, true);
            });

            it("works too", () => {
                assertEq(true, true);
            });
        "#,
    )?;
    context.run_assert(
        "src/index.test.ts",
        0,
        "
            src/index.test.ts -> works ...
            src/index.test.ts -> works PASSED
            src/index.test.ts -> works too ...
            src/index.test.ts -> works too PASSED
        ",
    );
    Ok(())
}

#[test]
fn multiple_tests_last_failing() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";

            it("works", () => {
                assertEq(true, true);
            });

            it("fails", () => {
                assertEq(true, false);
            });
        "#,
    )?;
    context.run_assert(
        "src/index.test.ts",
        1,
        "
            src/index.test.ts -> works ...
            src/index.test.ts -> works PASSED
            src/index.test.ts -> fails ...
            true
                !==
            false
            src/index.test.ts -> fails FAILED
        ",
    );
    Ok(())
}

#[test]
fn multiple_tests_first_failing() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";

            it("fails", () => {
                assertEq(true, false);
            });

            it("works", () => {
                assertEq(true, true);
            });
        "#,
    )?;
    context.run_assert(
        "src/index.test.ts",
        1,
        "
            src/index.test.ts -> fails ...
            true
                !==
            false
            src/index.test.ts -> fails FAILED
            src/index.test.ts -> works ...
            src/index.test.ts -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn multiple_failing_tests() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";

            it("fails", () => {
                assertEq(true, false);
            });

            it("fails too", () => {
                assertEq(true, false);
            });
        "#,
    )?;
    context.run_assert(
        "src/index.test.ts",
        1,
        "
            src/index.test.ts -> fails ...
            true
                !==
            false
            src/index.test.ts -> fails FAILED
            src/index.test.ts -> fails too ...
            true
                !==
            false
            src/index.test.ts -> fails too FAILED
        ",
    );
    Ok(())
}

#[test]
fn jsx_and_tsx() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.tsx",
        r#"
            import { assertEq, it } from "str";
            import { Foo } from "./foo";
            import * as React from "react";
            it("works", () => {
                const foo = <Foo />;
            });
        "#,
    )?;
    context.write(
        "src/foo.jsx",
        r#"
            export const Foo = () => <div> foo </div>;
        "#,
    )?;
    context.run_assert(
        "src/index.test.tsx",
        0,
        "
            src/index.test.tsx -> works ...
            src/index.test.tsx -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn local_imports() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import { foo } from "./foo";
            it("works", () => {
                assertEq(2, foo(1, 1));
            });
        "#,
    )?;
    context.write(
        "foo.ts",
        r#"
            export function foo(a: number, b: number) {
                return a + b;
            }
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        "
            index.test.ts -> works ...
            index.test.ts -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn local_imports_with_tsx_extension() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import { foo } from "./foo";
            it("works", () => {
                assertEq(2, foo(1, 1));
            });
        "#,
    )?;
    context.write(
        "foo.tsx",
        r#"
            export function foo(a: number, b: number) {
                return a + b;
            }
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        "
            index.test.ts -> works ...
            index.test.ts -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn local_imports_in_subdirectories() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import { foo } from "./subdir/foo";
            it("works", () => {
                assertEq(2, foo(1, 1));
            });
        "#,
    )?;
    context.write(
        "subdir/foo.ts",
        r#"
            export function foo(a: number, b: number) {
                return a + b;
            }
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        "
            index.test.ts -> works ...
            index.test.ts -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn local_imports_of_index_files() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import { foo } from "./subdir";
            it("works", () => {
                assertEq(2, foo(1, 1));
            });
        "#,
    )?;
    context.write(
        "subdir/index.ts",
        r#"
            export function foo(a: number, b: number) {
                return a + b;
            }
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        "
            index.test.ts -> works ...
            index.test.ts -> works PASSED
        ",
    );
    Ok(())
}

#[test]
fn errors_contain_source_location() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import { something } from "missing";
            import { somethingElse } from "./also_missing";
            it("works", () => {
                assertEq(something, somethingElse);
            });
        "#,
    )?;
    let stderr = context.run_assert_stderr("index.test.ts", 1);
    assert_contains(&stderr, "Could not resolve \"missing\"");
    assert_contains(&stderr, "index.test.ts:3:38");
    assert_contains(&stderr, "Could not resolve \"./also_missing\"");
    assert_contains(&stderr, "index.test.ts:4:42");
    Ok(())
}

#[test]
fn errors_in_dependencies_contain_source_location() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import { foo } from "./foo";
            it("works", () => {
                assertEq(2, foo(1, 1));
            });
        "#,
    )?;
    context.write(
        "foo.ts",
        r#"
            import { something } from "./missing";
            export function foo(a, b) {
                return something(a, b);
            }
        "#,
    )?;
    let stderr = context.run_assert_stderr("index.test.ts", 1);
    assert_contains(&stderr, "Could not resolve \"./missing\"");
    assert_contains(&stderr, "foo.ts:2:38");
    Ok(())
}

#[test]
fn bundling_errors_mean_node_will_not_be_run() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { something } from "missing";
            something();
        "#,
    )?;
    let stderr = context.run_assert_stderr("index.test.ts", 1);
    assert_contains(&stderr, "Could not resolve \"missing\"");
    assert_contains(&stderr, "index.test.ts:2:38");
    assert!(!stderr.contains("node"));
    Ok(())
}

#[test]
fn reexport_ts_types() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            export { T } from "./dependency";
            it("works", () => {
                let x: T = true;
                assertEq(x, true);
            });
        "#,
    )?;
    context.write(
        "dependency.ts",
        r#"
            export type T = boolean;
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

#[test]
fn __dirname_works_as_intended() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it } from "str";
            it("test", () => {
                console.error(`__dirname: ${__dirname}`);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        &format!(
            r#"
                index.test.ts -> test ...
                __dirname: {}
                index.test.ts -> test PASSED
            "#,
            context.temp_dir.path().to_string_lossy(),
        ),
    );
    Ok(())
}

#[test]
fn does_not_create_other_files_or_directories() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            it("works", () => {
                assertEq(true, true);
            });
        "#,
    )?;
    let StdoutUntrimmed(before) = context.run_command("ls");
    context.run_assert_stderr("index.test.ts", 0);
    let StdoutUntrimmed(after) = context.run_command("ls");
    assert_eq!(before, after);
    Ok(())
}

#[test]
fn node_apis() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import { basename } from "path";
            console.error(basename("dir/file"));
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        "
            file
        ",
    );
    Ok(())
}
