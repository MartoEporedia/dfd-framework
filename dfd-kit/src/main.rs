use clap::Parser;
use dfd_kit::{cli::Cli, execute};

fn main() {
    match execute(Cli::parse()) {
        Ok((output, code)) => {
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
            std::process::exit(code);
        }
        Err(error) => {
            eprintln!("DFD: {error}");
            std::process::exit(1);
        }
    }
}
