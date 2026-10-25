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
        default_value = "wireguard.json"
    )]
    pub config: PathBuf,

    #[arg(
        short,
        long,
        help = "Output directory location",
        default_value = "wireguard"
    )]
    pub output: PathBuf,

    #[arg(short, long, help = "Export peers")]
    pub export: bool,

    #[arg(short, long, help = "Generate ULA prefix")]
    pub generate: bool,

    #[arg(help = "Peer name", value_name = "NAME")]
    pub names: Vec<String>,
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}
