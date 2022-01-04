mod test_runner;
mod ts_to_js;

use crate::test_runner::TestRunner;
use std::path::Path;

fn main() {
    let test_files = std::env::args().skip(1).collect::<Vec<_>>();
    let test_file = test_files.get(0).expect("fixme");
    TestRunner::new().run_test_file(Path::new(test_file));
}
