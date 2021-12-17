#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use anyhow::Result;
    use cradle::prelude::*;
    use std::fs;
    use std::fs::create_dir_all;
    use std::path::Path;
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
        temp_dir: TempDir,
    }

    impl Context {
        fn new() -> Result<Self> {
            let temp_dir = TempDir::new()?;
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
            let (Stderr(stderr), Status(status)) = ("../str", file).run_output();
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
                it("fails", () => {
                  assertEq(true, false);
                });
            "#,
        )?;
        let result = context.run("src/index.test.ts");
        assert_eq!(result.status.code(), Some(1));
        assert_contains(result.stderr, "true\n    !=\nfalse".to_string());
        Ok(())
    }
}
