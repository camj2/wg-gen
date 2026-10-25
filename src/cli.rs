use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    author,
    about,
    help_template = "\
{name} {version}
{author}

{about}

{usage-heading} {usage}

{all-args}
"
)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "Config file location",
        default_value = "./wireguard.json"
    )]
    pub config: PathBuf,

    #[arg(
        short,
        long,
        help = "Output directory location",
        default_value = "./wireguard"
    )]
    pub output: PathBuf,

    #[arg(help = "Generate named peers.", value_name = "NAME")]
    pub names: Vec<String>,

    #[arg(short, long, help = "Generate unnamed peers.")]
    pub total: Option<u64>,

    #[arg(short, long, help = "Export peers to output directory.")]
    pub export: bool,

    #[arg(short, long, help = "Generate ULA prefix and exit.")]
    pub generate: bool,
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}
