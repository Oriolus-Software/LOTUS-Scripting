use clap::Parser;

mod commands;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcmd: commands::Commands,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt::init();

    cli.subcmd.execute()
}
