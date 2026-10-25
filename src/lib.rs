pub mod wireguard;
use wireguard::{Config, net::Ipv6Prefix};

pub mod cli;
use cli::Args;

// Thought: anyhow::Result<()>, thiserror or custom Error type (research)
pub fn run() -> anyhow::Result<()> {
    let args = Args::get();

    // Generate prefix, print and exit.
    if args.generate {
        let prefix = Ipv6Prefix::new().generate_ula();
        println!("{}", prefix);
        return Ok(());
    }

    // Deserialize existing config file, otherwise initialize new config.
    let mut config = if args.config.exists() {
        Config::from(&args.config)?
    } else {
        Config::new()
    };

    // Add named peers to config.
    for name in args.names {
        config.add(&name)?;
        println!("Added '{}'.", &name);
    }

    // Add unnamed peers to config.
    for total in args.total {
        let range = u64::MIN..total;

        for name in range {
            config.add(&name)?;
            println!("Added '{}'.", &name);
        }
    }

    // Delete peers.
    for name in args.delete {
        config.del(&name)?;
        println!("Deleted '{}'.", &name);
    }

    // Display config.
    println!("{}", &config);

    // Save config file.
    if args.write {
        config.write(&args.config)?;
        println!("Successfully generated peers!");
    }

    // Generate WireGuard config files.
    if args.export {
        config.export(&args.output)?;
        println!("Successfully exported peers!");
    }

    Ok(())
}
