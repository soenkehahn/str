use anyhow::anyhow;
use anyhow::Result;
use cradle::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;
use std::fs::create_dir_all;
use std::os::unix;
use std::path::Path;
use std::process::ExitStatus;
use tempfile::TempDir;
use unindent::Unindent;

struct Context {
    temp_dir: TempDir,
}

impl Context {
    fn new() -> Result<Self> {
        let repo_dir = std::env::current_dir()?;
        let temp_dir = TempDir::new()?;
        ("mkdir", temp_dir.path().join("node_modules/")).run();
        for dependency in fs::read_dir(repo_dir.join("tests/test-project/node_modules"))? {
            let dependency = dependency?.path();
            unix::fs::symlink(
                &dependency,
                temp_dir
                    .path()
                    .join("node_modules/")
                    .join(dependency.file_name().unwrap()),
            )?;
        }
        Ok(Context { temp_dir })
    }

    fn write<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<()> {
        let file = self.temp_dir.path().join(path.as_ref());
        let dir = file.parent().ok_or(anyhow!("no parent"))?;
        create_dir_all(&dir)?;
        fs::write(file, content)?;
        Ok(())
    }

    fn run(&self, file: &str) -> Output {
        let (Stderr(stderr), Status(status)) =
            self.run_command((executable_path::executable_path("str"), file));
        eprintln!("STDERR:\n{}STDERR END", stderr);
        Output { status, stderr }
    }

    fn run_command<I: Input, O: cradle::Output>(&self, i: I) -> O {
        let (StdoutUntrimmed(stdout), o) = (CurrentDir(self.temp_dir.path()), i).run_output();
        print!("{}", stdout);
        o
    }

    fn run_assert(&self, file: &str, expected_exit_code: i32, expected_stderr: &str) {
        let output = self.run(file);
        assert_eq!(output.status.code(), Some(expected_exit_code));
        assert_eq!(output.stderr, expected_stderr.unindent());
    }
}

struct Output {
    status: ExitStatus,
    stderr: String,
}

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
fn test_modules_have_same_base_names() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/foo.ts",
        r#"
            import { assertEq, it } from "str";
            import { fileURLToPath } from 'url'
            import { basename, dirname, extname } from 'path'

            it("has the basename foo", () => {
                let path = fileURLToPath(import.meta.url);
                const filename = basename(path, extname(path));
                assertEq("foo", filename);
            });
        "#,
    )?;
    let result = context.run("src/foo.ts");
    assert_eq!(result.status.code(), Some(0));
    Ok(())
}

#[test]
fn jsx() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "src/index.test.ts",
        r#"
            import { assertEq, it } from "str";
            import * as React from "react";
            it("works", () => {
                const App = () => <div> foo </div>;
                const app = <App />;
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
