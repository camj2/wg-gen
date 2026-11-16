mod cli;
pub use cli::Args;

mod net;
pub use net::{IpPrefix, Ipv4Prefix, Ipv6Prefix};

mod error;
pub use error::{Error, Result};

mod peer;
pub use peer::Config;

pub fn run() -> Result<()> {
    let args = Args::get();

    if args.generate {
        let prefix = Ipv6Prefix::default().generate_ula();
        println!("{}", prefix);
        return Ok(());
    }

    let mut config = if args.config.exists() {
        Config::from_path(&args.config)?
    } else {
        Config::new()
    };

    for name in args.delete {
        config.del(&name)?;
        println!("Deleted '{}'.", &name);
    }

    for name in args.names {
        config.add(&name)?;
        println!("Added '{}'.", &name);
    }

    if let Some(total) = args.total {
        for name in u64::MIN..total {
            config.add(&name)?;
            println!("Added '{}'.", &name);
        }
    }

    config.print();

    if args.write {
        config.write_to_path(&args.config)?;
        println!("Successfully generated peers!");
    }

    if args.export {
        config.export_to_path(&args.output)?;
        println!("Successfully exported peers!");
    }

    Ok(())
}
