#![allow(dead_code)]

use anyhow::anyhow;
use anyhow::Result;
use cradle::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;
use std::fs::create_dir_all;
use std::os::unix;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitStatus;
use tempfile::TempDir;
use unindent::Unindent;

pub fn assert_contains<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) {
    assert!(
        a.as_ref().contains(b.as_ref()),
        "=========\n{:?},\n\ndid not contain\n\n{:?}\n=========\n",
        a.as_ref(),
        b.as_ref()
    );
}

#[derive(Debug)]
pub struct Context {
    pub temp_dir: TempDir,
    repo_dir: PathBuf,
}

impl Context {
    pub fn new() -> Result<Self> {
        let repo_dir = std::env::current_dir()?;
        let temp_dir = TempDir::new()?;
        ("mkdir", temp_dir.path().join("node_modules/")).run();
        for dependency in fs::read_dir(repo_dir.join("tests/test-project/node_modules"))? {
            let dependency = dependency?.path();
            unix::fs::symlink(
                &dependency,
                temp_dir
                    .path()
                    .join("node_modules/")
                    .join(dependency.file_name().unwrap()),
            )?;
        }
        Ok(Context { temp_dir, repo_dir })
    }

    pub fn write<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<()> {
        let file = self.temp_dir.path().join(path.as_ref());
        let dir = file.parent().ok_or(anyhow!("no parent"))?;
        create_dir_all(&dir)?;
        fs::write(file, content)?;
        Ok(())
    }

    pub fn read<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let file = self.temp_dir.path().join(path.as_ref());
        Ok(fs::read_to_string(file)?)
    }

    pub fn run(&self, args: &str) -> Output {
        let (Stderr(stderr), Status(status)) = self.run_command((
            self.repo_dir.join("str"),
            args.split_whitespace().collect::<Vec<&str>>(),
        ));
        eprintln!("STDERR:\n{}STDERR END", stderr);
        Output { status, stderr }
    }

    pub fn run_command<I: Input, O: cradle::Output>(&self, i: I) -> O {
        let (StdoutUntrimmed(stdout), o) = (CurrentDir(self.temp_dir.path()), i).run_output();
        print!("{}", stdout);
        o
    }

    pub fn run_assert(
        &self,
        args: &str,
        expected_exit_code: i32,
        expected_stderr: &str,
    ) -> Result<()> {
        let stderr = self.run_assert_stderr(args, expected_exit_code);
        assert_eq!(
            strip_ansi(&stderr)?.lines().collect::<Vec<_>>(),
            expected_stderr.unindent().lines().collect::<Vec<_>>()
        );
        Ok(())
    }

    pub fn run_assert_with_colors(
        &self,
        args: &str,
        expected_exit_code: i32,
        expected_stderr: &str,
    ) -> Result<()> {
        let stderr = self.run_assert_stderr(args, expected_exit_code);
        assert_eq!(
            stderr.lines().collect::<Vec<_>>(),
            expected_stderr.unindent().lines().collect::<Vec<_>>()
        );
        Ok(())
    }

    pub fn run_assert_stderr(&self, args: &str, expected_exit_code: i32) -> String {
        let output = self.run(args);
        assert_eq!(output.status.code(), Some(expected_exit_code));
        output.stderr
    }
}

pub struct Output {
    status: ExitStatus,
    stderr: String,
}

pub fn strip_ansi(input: &str) -> Result<String> {
    Ok(String::from_utf8(strip_ansi_escapes::strip(input)?)?)
}
