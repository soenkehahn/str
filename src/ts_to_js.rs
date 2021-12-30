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

pub fn ts_to_js(test_file: &Path) -> String {
    let spans: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(spans.clone()));
    let file = spans.load_file(test_file).expect("failed to load file");
    let compiler = swc::Compiler::new(spans);
    GLOBALS.set(compiler.globals(), || -> String {
        let syntax = EsVersion::Es2015;
        let program = compiler
            .parse_js(
                file,
                &handler,
                syntax,
                Syntax::Typescript(Default::default()),
                IsModule::Bool(true),
                true,
            )
            .expect("fixme");
        let program = compiler.transform(
            &handler,
            program,
            false,
            swc_ecma_transforms_typescript::strip::strip(Mark::fresh(Mark::root())),
        );
        compiler
            .print(
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
            )
            .expect("fixme")
            .code
    })
}
