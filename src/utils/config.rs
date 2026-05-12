use crate::utils::terminal::{green, yellow};

#[derive(Debug, Clone)]
pub struct CliConfig {
    pub verbose: bool,
    pub seed: bool,
}

impl CliConfig {
    pub fn from_args() -> Self {
        let mut verbose: bool = false;
        let mut seed: bool = false;

        for arg in std::env::args().skip(1) {
            match arg.as_str() {
                "--verbose" | "-v" => verbose = true,
                "--seed" | "-s" => seed = true,
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => eprintln!("{} Unknown argument ignored: {}", yellow("[WARN]"), arg),
            }
        }

        if verbose {
            println!("{} Verbose mode enabled", green("[OK]"));
        }

        if seed {
            println!(
                "{} Seed mode enabled: default user and pricing will be created if not already present",
                green("[OK]")
            );
        }

        Self { verbose, seed }
    }
}

fn print_help() {
    println!(
        "\
Usage:
  api [options]

Options:
  -v, --verbose    Enable verbose request and step timing logs
  -h, --help       Show this help message
"
    );
}
