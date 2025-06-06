use clap::Parser;

mod cli;
mod rune;

fn main() {
    let cargo = cli::CargoCli::parse();
    let cli::CargoCommands::Rune(args) = cargo.command;

    let command = args.command.or_else(|| {
        args.run_args.map(|inner| cli::Commands::Run(inner))
    }).unwrap();

    match command {
        cli::Commands::Run(run_args) => with_tokio(rune::run_script(run_args.input)),
    }
}

fn with_tokio<F: Future>(future: F) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(future);
}
