pub mod wireguard;
use wireguard::{Config, net::Ipv6Prefix};

pub mod cli;
use cli::Args;

// Thought: anyhow::Result<()>, thiserror or custom Error type (research)
pub fn run() {
    let args = Args::get();

    if args.generate {
        let prefix = Ipv6Prefix::new().generate_ula();
        println!("{}", prefix);
        // return Ok(());
        return;
    }

    if !args.names.is_empty() {
        let mut config = Config::from(args.names);
        println!("Successfully generated peers!");
        config.write(args.config);
        // config.export(args.output);
        // return Ok(());
        return;
    }

    if args.export {
        let config = Config::from_path(args.config);

        config.export(args.output);
        return;
    }

    eprintln!("Nothing to do.");
    // Ok(())
}
