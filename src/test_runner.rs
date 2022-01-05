use crate::bundler::Bundler;
use anyhow::anyhow;
use anyhow::Result;
use cradle::prelude::*;
use std::os::unix;
use std::path::Path;
use std::process::ExitStatus;
use tempfile::TempDir;
use unindent::Unindent;

pub struct TestRunner {
    temp_dir: TempDir,
}

impl TestRunner {
    pub fn new() -> Result<Self> {
        Ok(TestRunner {
            temp_dir: TempDir::new()?,
        })
    }

    pub fn run_test_file(&self, test_file: &Path) -> Result<()> {
        unix::fs::symlink(
            std::env::current_dir()?.join("node_modules"),
            self.temp_dir.path().join("node_modules"),
        )?;
        let status = self.run_as_module(test_file)?;
        if status.success() {
            std::process::exit(0);
        } else {
            match status.code() {
                Some(code) => std::process::exit(code),
                None => std::process::exit(1),
            }
        }
    }

    fn run_as_module(&self, test_file: &Path) -> Result<ExitStatus> {
        std::fs::create_dir_all("str-dist")?;
        let runner_file = Path::new("str-dist/runner.mjs");
        std::fs::write(&runner_file, TestRunner::runner_code(test_file)?)?;
        let js_file = Bundler::bundle(runner_file, self.temp_dir.path())?;
        let Status(status) = ("node", js_file).run_output();
        Ok(status)
    }

    fn runner_code(test_file: &Path) -> Result<String> {
        let test_file = path_to_str(test_file)?;
        Ok(format!(
            "
                import {{ _strTestRunner }} from \"str\";
                async function main() {{
                    _strTestRunner.setTestFile(\"{}\");
                    await import(\"../{}\");
                    _strTestRunner.finalize();
                }}
                main();
            ",
            test_file, test_file
        )
        .unindent())
    }
}

fn path_to_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("cannot convert to string: {:?}", path))
}
