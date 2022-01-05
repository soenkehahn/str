use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use lexiclean::Lexiclean;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use swc::config::util::BoolOrObject;
use swc::config::IsModule;
use swc::config::SourceMapsConfig;
use swc_common::chain;
use swc_common::errors::ColorConfig;
use swc_common::errors::Handler;
use swc_common::hygiene::Mark;
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_common::GLOBALS;
use swc_ecma_ast::EsVersion;
use swc_ecma_ast::ImportDecl;
use swc_ecma_parser::Syntax;
use swc_ecma_parser::TsConfig;
use swc_ecma_visit::VisitMut;
use swc_ecma_visit::VisitMutWith;

pub struct Imports {
    queue: VecDeque<Result<PathBuf>>,
    current_dir: PathBuf,
}

impl Imports {
    pub fn run(main_file: &Path, output_dir: &Path) -> Result<PathBuf> {
        let mut imports = Self::new(main_file)?;
        imports.push(Ok(main_file.to_owned()));
        while let Some(file) = imports.pop()? {
            let output_file = Self::get_output_file(output_dir, &file)?;
            if let Some(dir) = output_file.parent() {
                fs::create_dir_all(dir)?;
            }
            fs::write(&output_file, imports.convert_to_js(&file)?).context(anyhow!(
                "cannot write to \"{}\"",
                output_file.to_string_lossy()
            ))?;
        }
        Self::get_output_file(output_dir, main_file)
    }

    fn file_parent(file: &Path) -> Result<PathBuf> {
        Ok(file
            .parent()
            .ok_or(anyhow!("source code file has no parent"))?
            .to_owned())
    }

    fn get_output_file(output_dir: &Path, file: &Path) -> Result<PathBuf> {
        Ok(output_dir.join(file).with_extension("mjs"))
    }

    fn new(current_file: &Path) -> Result<Self> {
        Ok(Self {
            queue: VecDeque::new(),
            current_dir: Self::file_parent(current_file)?,
        })
    }

    fn convert_to_js(&mut self, path: &Path) -> Result<String> {
        let spans: Lrc<SourceMap> = Default::default();
        let handler =
            Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(spans.clone()));
        let file = spans
            .load_file(path)
            .context(format!("cannot load {}", path.to_string_lossy()))?;
        let compiler = swc::Compiler::new(spans.clone());
        GLOBALS.set(compiler.globals(), || -> Result<String> {
            let syntax = EsVersion::Es2015;
            let program = compiler.parse_js(
                file,
                &handler,
                syntax,
                Syntax::Typescript(TsConfig {
                    tsx: true,
                    ..Default::default()
                }),
                IsModule::Bool(true),
                true,
            )?;
            let top_level_mark = Mark::fresh(Mark::root());
            let transforms = chain!(
                swc_ecma_transforms_react::jsx(
                    spans,
                    Some(swc_common::comments::NoopComments),
                    Default::default(),
                    top_level_mark,
                ),
                swc_ecma_transforms::resolver_with_mark(top_level_mark),
                swc_ecma_transforms_typescript::strip::strip(top_level_mark),
            );
            let mut program = compiler.transform(&handler, program, false, transforms);
            program.visit_mut_with(self);
            let output = compiler.print(
                &program,
                Some(path.to_string_lossy().as_ref()),
                None,
                false,
                syntax,
                SourceMapsConfig::Bool(false),
                &Default::default(),
                None,
                false,
                Some(BoolOrObject::Bool(true)),
            )?;
            Ok(output.code)
        })
    }

    fn push(&mut self, path: Result<PathBuf>) {
        self.queue.push_front(path);
    }

    fn pop(&mut self) -> Result<Option<PathBuf>> {
        self.queue.pop_front().transpose()
    }

    fn resolve_local_import(&self, import_string: &str) -> ImportedFile {
        if import_string.starts_with("./") {
            let module_extensions = vec!["ts", "tsx"];
            let candidates = module_extensions.into_iter().map(|extension| {
                self.current_dir
                    .join(PathBuf::from(format!("{}.{}", import_string, extension)))
                    .lexiclean()
            });
            for candidate in candidates {
                if candidate.exists() {
                    return ImportedFile::LocalFile(Ok(candidate));
                }
            }
            ImportedFile::LocalFile(Err(anyhow!("cannot find module \"{}\"", import_string)))
        } else {
            ImportedFile::NonLocalFile
        }
    }
}

enum ImportedFile {
    LocalFile(Result<PathBuf>),
    NonLocalFile,
}

impl VisitMut for Imports {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        let import_string: &str = import_decl.src.value.as_ref();
        match self.resolve_local_import(import_string) {
            ImportedFile::LocalFile(dependency_file) => {
                self.queue.push_front(dependency_file);
            }
            ImportedFile::NonLocalFile => {}
        }
    }
}
