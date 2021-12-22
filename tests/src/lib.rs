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
            let repo_dir = std::env::current_dir()?
                .parent()
                .ok_or(anyhow!("$REPO/tests has no parent"))?
                .to_owned();
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
            run!(%"yarn install --offline", CurrentDir(temp_dir.path()));
            run!(
                "yarn",
                "add",
                format!(
                    "link:{}",
                    repo_dir.to_str().ok_or(anyhow!("invalid utf-8"))?
                ),
                "--offline",
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
                self.repo_dir.join("str"),
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
