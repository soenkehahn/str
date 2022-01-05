mod bundler;
mod test_runner;

use crate::test_runner::TestRunner;
use anyhow::anyhow;
use anyhow::Result;
use std::path::Path;

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("ERROR: {}", error);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let test_files = std::env::args().skip(1).collect::<Vec<_>>();
    let test_file = test_files.get(0).ok_or(anyhow!("no test file given"))?;
    TestRunner::new()?.run_test_file(Path::new(test_file))?;
    Ok(())
}
