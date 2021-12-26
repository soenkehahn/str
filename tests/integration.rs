use anyhow::Result;
use cradle::prelude::*;

fn assert_contains<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) {
    assert!(
        a.as_ref().contains(b.as_ref()),
        "\nassert_contains(\n  {:?},\n  {:?}\n)\n",
        a.as_ref(),
        b.as_ref()
    );
}

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
            "podman",
            "run",
            "--rm",
            image.to_owned(),
            "str",
            file.to_owned(),
        )
    }
    let (Status(status), Stderr(output)) = run_command(&image, "failing.test.ts").run_result()?;
    assert_eq!(status.code(), Some(1));
    assert_contains(output, "true\n    !==\nfalse");
    let (Status(status), Stderr(output)) = run_command(&image, "passing.test.ts").run_result()?;
    assert!(status.success());
    assert_eq!(output, "");
    Ok(())
}
