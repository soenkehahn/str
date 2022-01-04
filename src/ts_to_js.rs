use anyhow::Result;
use std::path::Path;
use swc::config::util::BoolOrObject;
use swc::config::IsModule;
use swc::config::SourceMapsConfig;
use swc_common::errors::ColorConfig;
use swc_common::errors::Handler;
use swc_common::hygiene::Mark;
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_common::GLOBALS;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::Syntax;
use swc_ecma_parser::TsConfig;
use swc_ecma_visit::Fold;

pub fn ts_to_js(test_file: &Path) -> Result<String> {
    let spans: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(spans.clone()));
    let file = spans.load_file(test_file)?;
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
        let transforms: Vec<Box<dyn Fold>> = vec![
            Box::new(swc_ecma_transforms_react::jsx(
                spans,
                Some(swc_common::comments::NoopComments),
                Default::default(),
                top_level_mark,
            )),
            Box::new(swc_ecma_transforms::resolver_with_mark(top_level_mark)),
            Box::new(swc_ecma_transforms_typescript::strip::strip(top_level_mark)),
        ];
        let program = transforms.into_iter().fold(program, |program, transform| {
            compiler.transform(&handler, program, false, transform)
        });
        let output = compiler.print(
            &program,
            Some(test_file.to_string_lossy().as_ref()),
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
