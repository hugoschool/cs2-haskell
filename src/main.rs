mod build_systems;
mod ci;
mod commands;
mod package;
mod parse;
mod shared;

use ci::Ci;
use clap::{Parser, Subcommand};
use std::{
    io::{BufRead, IsTerminal},
    str::FromStr,
};

#[derive(Subcommand)]
enum ArgSubcommand {
    /// Installs all the dependencies needed
    Install {
        /// Only install a certain package
        #[arg(long)]
        package: Option<String>,
    },
    /// Update cs2 and the dependencies
    Update {
        /// Only update a certain package
        #[arg(long)]
        package: Option<String>,

        /// Force update even if there is nothing new when fetching
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<ArgSubcommand>,

    /// Prints the errors in a correct way for the specified platform
    #[arg(long)]
    ci: Option<String>,

    /// Disable checking for files ignored by git
    #[arg(long)]
    no_ignore: bool,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(ArgSubcommand::Install { package }) => {
            match commands::install::handler(package) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        Some(ArgSubcommand::Update { package, force }) => {
            match commands::update::handler(package, *force) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        &None => {
            let ci: Option<Ci> = if let Some(ci) = args.ci {
                match Ci::from_str(&ci) {
                    Ok(ci) => Some(ci),
                    Err(_) => {
                        println!("Incorrect CI platform, continuing.");
                        None
                    }
                }
            } else {
                None
            };

            if !std::io::stdin().is_terminal() && ci.is_none() {
                let mut full_input = Vec::new();
                for line in std::io::stdin().lock().lines() {
                    match line {
                        Ok(s) => full_input.push(s),
                        Err(_) => break,
                    }
                }

                let _ = parse::parse_output(full_input, true, None);
            } else {
                if !build_systems::verify_packages() {
                    println!(
                        "Some packages seem to not be installed, make sure you ran cs2-haskell install before"
                    );
                    std::process::exit(1);
                }

                let lines = match build_systems::find() {
                    Ok(lines) => lines,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                match parse::parse_output(lines, args.no_ignore, ci) {
                    Ok(exit) => {
                        if exit {
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };
            }
        }
    }
}
