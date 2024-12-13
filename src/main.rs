mod cli;
mod walkdir;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    // You can now use the CLI args like this:
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is definitely on"),
        _ => println!("Don't be crazy"),
    }
}
