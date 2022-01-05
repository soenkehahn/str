use anyhow::anyhow;
use anyhow::Result;
use esbuild_rs::build_direct;
use esbuild_rs::BuildOptions;
use esbuild_rs::BuildOptionsBuilder;
use esbuild_rs::BuildResult;
use esbuild_rs::Format;
use esbuild_rs::*;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Arc;

pub struct Bundler;

impl Bundler {
    pub fn bundle(main_file: &Path, output_dir: &Path) -> Result<PathBuf> {
        let mut options_builder = BuildOptionsBuilder::new();
        options_builder
            .entry_points
            .push(main_file.to_str().expect("fixme").to_owned());
        options_builder.outfile = output_dir.join("main.js").to_str().unwrap().to_owned();
        options_builder.bundle = true;
        options_builder.write = true;
        options_builder.format = Format::CommonJS;
        options_builder.platform = Platform::Node;
        options_builder.main_fields = vec!["main".to_owned()];
        options_builder.resolve_extensions = vec![".js", ".ts", ".tsx"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        let options = options_builder.build();
        build_direct_sync(options)?;
        Ok(output_dir.join("main.js"))
    }
}

fn build_direct_sync(options: Arc<BuildOptions>) -> Result<BuildResult> {
    let (sender, receiver) = channel();
    build_direct(options, move |build_result| {
        sender.send(build_result).unwrap()
    });
    let build_result = receiver.recv()?;
    for warning in build_result.warnings.as_slice() {
        eprintln!("{}", warning);
    }
    let errors = build_result.errors.as_slice();
    if !errors.is_empty() {
        return Err(anyhow!(
            "{}",
            errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    Ok(build_result)
}
