use std::fs;

use clap::{Parser, Subcommand};
use colored::Colorize;
use rustfetch::{DEFAULT_LUA_CONFIG, TUX_ASCII_ART};

#[derive(Parser)]
#[command(
    author = "xshotss",
    version = "v0.1.0",
    about = "A modern and highly customizable system information tool.",
    long_about = None,
)]
pub struct RustfetchCLI {
    #[arg(short, long, global = true, default_value_t = false)]
    verbose: bool,

    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Version,
    GenConfig,
}

fn main() {
    let cli = RustfetchCLI::parse();

    match &cli.commands {
        Some(n) => match n {
            Commands::GenConfig => {
                println!("Generating config files...");
                generate_config();
            }

            Commands::Version => {
                if cli.verbose {
                    print!("Rustfetch: ");
                    println!("A modern and highly customizable system information tool.");
                    println!("GitHub Repo: https://github.com/xshotss/rustfetch");
                }

                println!("v{}", env!("CARGO_PKG_VERSION"));
            }
        },

        None => {
            println!("This is not implemented yet!");
        }
    }
}

// generates ~/.config/rustfetch directory and some files
fn generate_config() {
    // gets user $HOME
    if let Some(home) = std::env::home_dir() {
        let destination = home.join(".config/rustfetch");
        match fs::create_dir(&destination) {
            Ok(_) => {
                println!(
                    "{} {}",
                    "Config directory created at {}".green(),
                    destination.display()
                );

                // generate config.lua file
                match std::fs::write(destination.join("config.lua"), DEFAULT_LUA_CONFIG) {
                    Ok(_) => println!("{}", "Generated default Lua file successfully!".green()),

                    Err(e) => {
                        eprintln!("{}", "CRITICAL: Failed to create default Lua file!".red());
                        eprintln!("Generated error: {e}");
                        std::process::exit(1);
                    }
                }

                // creates ascii directory
                match std::fs::create_dir(destination.join("ascii")) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!(
                            "{}",
                            "CRITICAL: Failed to create default ASCII directory!".red()
                        );
                        eprintln!("Generated error: {e}");
                        std::process::exit(1);
                    }
                }

                match std::fs::write(destination.join("ascii/tux.txt"), TUX_ASCII_ART) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{}", "CRITICAL: Failed to load Tux ASCII art!".red());
                        eprintln!("Generated error: {e}");
                        std::process::exit(1);
                    }
                }
            }

            Err(e) => {
                eprintln!(
                    "{} {}",
                    "Failed to create directory in {}!!".red(),
                    destination.display()
                );
                eprintln!("{} {}", "Error: {}\nAborting...".red(), e);
                std::process::exit(1);
            }
        }
    } else {
        // this can happen if $HOME is not configured
        // this is VERY unlikely to happen on like 99.9999% of linux systems
        eprintln!(
            "{}",
            "CRITICAL: Could not find user home directory!
        You're either not on a Linux system, or you do not have home directories configured.
        Aborting..."
                .red()
        );
        std::process::exit(1);
    }
}
