use cradle::prelude::*;
use std::fs;

fn main() {
    let test_files = std::env::args().skip(1).collect::<Vec<_>>();
    let test_file = test_files.get(0).unwrap(); // fixme
    let code = fs::read(test_file).expect("fixme");
    let Status(status) = ("node", "--input-type=module", Stdin(code)).run_output();
    if !status.success() {
        match status.code() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use anyhow::Result;
    use cradle::prelude::*;
    use std::fs;
    use std::fs::create_dir_all;
    use std::path::Path;
    use std::path::PathBuf;
    use std::process::ExitStatus;
    use tempfile::TempDir;

    fn assert_contains<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) {
        assert!(
            a.as_ref().contains(b.as_ref()),
            "\nassert_contains(\n  {:?},\n  {:?}\n)\n",
            a.as_ref(),
            b.as_ref()
        );
    }

    struct Context {
        repo_dir: PathBuf,
        temp_dir: TempDir,
    }

    impl Context {
        fn new() -> Result<Self> {
            let repo_dir = std::env::current_dir()?;
            let temp_dir = TempDir::new()?;
            fs::write(
                temp_dir.path().join("package.json"),
                r#"
                    {
                        "name": "str-test",
                        "version": "0.0.0",
                        "private": true
                    }
                "#,
            )?;
            run!(LogCommand, "yarn", "install", CurrentDir(temp_dir.path()));
            run!(
                LogCommand,
                "yarn",
                "add",
                "--dev",
                format!(
                    "link:{}",
                    repo_dir
                        .join("typescript-library")
                        .to_str()
                        .ok_or(anyhow!("invalid utf-8"))?
                ),
                CurrentDir(temp_dir.path())
            );
            Ok(Context { temp_dir, repo_dir })
        }

        fn write<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<()> {
            let file = self.temp_dir.path().join(path.as_ref());
            let dir = file.parent().ok_or(anyhow!("no parent"))?;
            create_dir_all(&dir)?;
            fs::write(file, content)?;
            Ok(())
        }

        fn run(&self, file: &str) -> Output {
            let (Stderr(stderr), Status(status)) = (
                "cargo",
                "run",
                "--manifest-path",
                self.repo_dir.join("Cargo.toml"),
                file,
                CurrentDir(self.temp_dir.path()),
            )
                .run_output();
            eprintln!("STDERR:\n{}", stderr);
            Output { status, stderr }
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
        assert_contains(result.stderr, "true\n    !==\nfalse".to_string());
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
}
