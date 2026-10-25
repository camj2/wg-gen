pub mod wireguard;
use wireguard::{Config, net::Ipv6Prefix};

pub mod cli;
use cli::Args;

// Thought: anyhow::Result<()>, thiserror or custom Error type (research)
pub fn run() -> anyhow::Result<()> {
    let args = Args::get();

    if args.generate {
        let prefix = Ipv6Prefix::new().generate_ula();
        println!("{}", prefix);
        return Ok(());
    }

    if !args.names.is_empty() {
        if args.config.exists() {
            eprintln!("Already generated peers.");
            return Ok(());
        }
        let config = Config::from(args.names);
        config.write(args.config)?;
        println!("Successfully generated peers!");
        return Ok(());
    }

    if args.export {
        let config = Config::from_path(args.config)?;
        config.export(args.output)?;
        println!("Successfully exported peers!");
        return Ok(());
    }

    eprintln!("Nothing to do.");
    Ok(())
}
