use std::fs;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    author = "xshotss",
    version = "v0.1.0",
    about = "A modern and highly customizable system information tool.",
    long_about = None,
)]
pub struct RustfetchCLI {
    #[command(subcommand)]
    pub commands: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    GenConfig,
}



fn main() {
    let cli = RustfetchCLI::parse();

    match &cli.commands {
        Some(n) => {
            match n {
                Commands::GenConfig => {
                    println!("Generating config files...");
                    generate_config();
                }


            }
        }

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
                println!("{} {}", "Config directory created at {}".green(),
                destination.display()
                );

                // generate config.lua file
                match std::fs::write(destination.join("config.lua"), "
-- This is an automatically generating config file for Rustfetch.
-- Check the Github repo for help:
-- https://github.com/xshotss/rustfetch

ascii_art = \"default.txt\"
modules = \"modules.json\"
                ") {
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
                        eprintln!("{}", "CRITICAL: Failed to create default ASCII directory!".red());
                        eprintln!("Generated error: {e}");
                        std::process::exit(1);
                    }
                }

                // creates modules.json
                match std::fs::write(destination.join("modules.json"), "
{
    \"modules\" = [\"cpu\", \"gpu\", \"host\"]
}
                ") {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{}", "CRITICAL: Failed to create modules JSON file!".red());
                        eprintln!("Generated error: {e}");
                        std::process::exit(1);
                    }
                }
            }

            Err(e) => {
                eprintln!("{} {}", "Failed to create directory in {}!!".red(), destination.display());
                eprintln!("{} {}", "Error: {}\nAborting...".red(), e);
                std::process::exit(1);
            }
        }
    }

    else {
        eprintln!("{}", "CRITICAL: Could not find user home directory!
        You're either not on a Linux system, or you do not have home directories configured.
        Aborting..."
        .red());
        std::process::exit(1);
    }
}


