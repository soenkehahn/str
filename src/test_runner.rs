use crate::ts_to_js::ts_to_js;
use cradle::prelude::*;
use std::fs;
use std::path::Path;
use std::process::ExitStatus;
use tempfile::TempDir;

pub struct TestRunner {
    temp_dir: TempDir,
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            temp_dir: TempDir::new().expect("fixme"),
        }
    }

    pub fn run_test_file(&self, test_file: &Path) -> ! {
        // fixme: don't use cradle for linking
        (
            "ln",
            "-s",
            std::env::current_dir().expect("fixme").join("node_modules"),
            self.temp_dir.path().join("node_modules"),
        )
            .run();
        let status = self.run_as_module(test_file);
        if status.success() {
            std::process::exit(0);
        } else {
            match status.code() {
                Some(code) => std::process::exit(code),
                None => std::process::exit(1),
            }
        }
    }

    fn run_as_module(&self, test_file: &Path) -> ExitStatus {
        let js_file = self.temp_dir.path().join(format!(
            "{}.mjs",
            test_file.file_stem().unwrap().to_str().unwrap()
        ));
        fs::write(&js_file, ts_to_js(test_file)).expect("fixme");
        let Status(status) = (
            "node",
            "--input-type=module",
            Stdin(TestRunner::runner_code(&js_file)),
        )
            .run_output();
        status
    }

    fn runner_code(test_js_file: &Path) -> String {
        format!(
            "
            import \"{}\";
            import {{ finalize }} from \"str\";
            finalize();
        ",
            &test_js_file.to_str().unwrap()
        )
    }
}
