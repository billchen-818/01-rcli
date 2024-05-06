mod base64;
mod csv;
mod genpass;

use self::csv::CsvOpts;
use self::genpass::GenPassOpts;
use clap::Parser;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "0.1", author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random passphrase")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("no-exist"), Err("File not found"));
    }
}
