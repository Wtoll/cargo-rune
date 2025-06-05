use std::{io::Read, sync::Arc};

use lazy_static::lazy_static;
use rune::{BuildError, Context, Diagnostics, Hash, Source, Sources, Unit, Vm};
use termcolor::{ColorChoice, StandardStream};

lazy_static! {
    static ref ENTRYPOINT: Hash = Hash::type_hash(["main"]);
}

pub async fn run_script(input: clio::Input) {
    let context = build_rune_context();

    let mut sources = Sources::new();
    sources.insert(source_from_cli(input)).unwrap();

    let unit = compile_sources(sources, &context).unwrap();

    execute_main(context, unit).await;
}

fn build_rune_context() -> Context {
    Context::with_default_modules().unwrap()
}

fn source_from_cli(mut input: clio::Input) -> Source {
    match input.get_file() {
        Some(file) => {
            let mut source = String::new();

            file.read_to_string(&mut source).unwrap();

            let name = input.path().file_stem().unwrap().to_str().unwrap();

            Source::with_path(name, source, input.path().path()).unwrap()
        },
        None => {
            let mut source = String::new();

            input.read_to_string(&mut source).unwrap();

            Source::new("stdin", source).unwrap()
        },
    }
}

fn compile_sources(mut sources: Sources, context: &Context) -> Result<Unit, BuildError> {
    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources).unwrap();
    }

    result
}

async fn execute_main(context: Context, unit: Unit) {
    let runtime = Arc::new(context.runtime().unwrap());
    let mut vm = Vm::new(runtime, Arc::new(unit));

    vm.async_call(*ENTRYPOINT, ()).await.unwrap();
}