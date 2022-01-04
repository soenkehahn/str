use crate::ts_to_js::ts_to_js;
use anyhow::anyhow;
use anyhow::Result;
use cradle::prelude::*;
use std::fs;
use std::os::unix;
use std::path::Path;
use std::process::ExitStatus;
use tempfile::TempDir;

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
        let js_file = self.temp_dir.path().join(format!(
            "{}.mjs",
            test_file
                .file_stem()
                .ok_or_else(|| anyhow!("no file stem: {:?}", test_file))?
                .to_str()
                .ok_or_else(|| anyhow!("cannot convert to string: {:?}", test_file))?
        ));
        fs::write(&js_file, ts_to_js(test_file)?)?;
        let Status(status) = (
            "node",
            "--input-type=module",
            Stdin(TestRunner::runner_code(test_file, &js_file)?),
        )
            .run_output();
        Ok(status)
    }

    fn runner_code(original_file: &Path, test_js_file: &Path) -> Result<String> {
        Ok(format!(
            "
                import {{ _strTestRunner }} from \"str\";
                async function main() {{
                    _strTestRunner.setTestFile(\"{}\");
                    await import(\"{}\");
                    _strTestRunner.finalize();
                }}
                main();
            ",
            path_to_str(original_file)?,
            path_to_str(test_js_file)?
        ))
    }
}

fn path_to_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("cannot convert to string: {:?}", path))
}
