use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub expression: String,

    #[arg(short, long)]
    pub precise: bool,
}
