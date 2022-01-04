use anyhow::anyhow;
use anyhow::Result;
use cradle::prelude::*;
use std::fs;
use std::fs::create_dir_all;
use std::os::unix;
use std::path::Path;
use std::process::ExitStatus;
use tempfile::TempDir;

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
    let result = context.run("src/index.test.ts");
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(result.stderr, "true\n    !==\nfalse\n".to_string());
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
    let result = context.run("src/index.test.ts");
    assert_eq!(result.status.code(), Some(0));
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
    let result = context.run("src/index.test.ts");
    assert_eq!(result.status.code(), Some(0));
    Ok(())
}

#[test]
fn multiple_succeeding_tests() -> Result<()> {
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
    let result = context.run("src/index.test.ts");
    assert_eq!(result.status.code(), Some(0));
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
    let result = context.run("src/index.test.ts");
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(result.stderr, "true\n    !==\nfalse\n".to_string());
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
    let result = context.run("src/index.test.ts");
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(result.stderr, "true\n    !==\nfalse\n".to_string());
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
    let result = context.run("src/index.test.ts");
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(
        result.stderr,
        "true\n    !==\nfalse\ntrue\n    !==\nfalse\n".to_string()
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
