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
    #[arg(help = "Generate named peers.", value_name = "NAME")]
    pub names: Vec<String>,

    #[arg(short, long, help = "Generate unnamed peers.")]
    pub total: Vec<u64>,

    #[arg(short, long, help = "Delete peer.", value_name = "NAME")]
    pub delete: Vec<String>,

    #[arg(short, long, help = "Write peers to config file.")]
    pub write: bool,

    #[arg(short, long, help = "Export peers to output directory.")]
    pub export: bool,

    #[arg(short, long, help = "Generate prefix, print and exit.")]
    pub generate: bool,

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
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}
