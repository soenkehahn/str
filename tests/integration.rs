mod common;

use anyhow::Result;
use common::strip_ansi;
use cradle::prelude::*;
use pretty_assertions::assert_eq;
use unindent::Unindent;

#[test]
fn integration_test() -> Result<()> {
    let build_command = (
        &LogCommand,
        "podman",
        "build",
        ("-f", "tests/from-scratch/Dockerfile"),
        ".",
    );
    build_command.run_result()?;
    let StdoutTrimmed(image) = (build_command, "--quiet").run_result()?;
    fn run_command(image: &str, file: &str) -> impl Input {
        (
            LogCommand,
            ("podman", "run"),
            "--rm",
            (
                "-v",
                format!(
                    "{}/str:/usr/local/bin/str",
                    std::env::current_dir().unwrap().to_str().unwrap()
                ),
            ),
            image.to_owned(),
            ("str", file.to_owned()),
        )
    }
    let (Status(status), Stderr(output)) = run_command(&image, "failing.test.ts").run_result()?;
    assert_eq!(status.code(), Some(1));
    assert_eq!(
        strip_ansi(&output)?,
        "
            failing.test.ts -> fails ...
            true
                !==
            false
            failing.test.ts -> fails FAILED
            Ran 1 test, 0 passed, 1 failed.
        "
        .unindent()
    );
    let (Status(status), Stderr(output)) = run_command(&image, "passing.test.ts").run_result()?;
    assert!(status.success());
    assert_eq!(
        strip_ansi(&output)?,
        "
            passing.test.ts -> passes ...
            passing.test.ts -> passes PASSED
            Ran 1 test, 1 passed, 0 failed.
        "
        .unindent()
    );
    Ok(())
}
