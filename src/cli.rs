use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "readme-checker")]
#[command(author = "Michael A Wright")]
#[command(version = "1.0.0")]
#[command(about = "Validates markdown files for UTF-8, ASCII-subset, and unprintable characters", long_about = None)]
pub struct Cli {
    /// Path to directory containing the file (default: current directory)
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Name of the file to check (default: README.md)
    #[arg(short = 'f', long, value_name = "NAME", default_value = "README.md")]
    pub file_name: String,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub filename: String,
    pub verbose: bool,
}

impl Config {
    pub fn from_cli(cli: Cli) -> Self {
        Self {
            path: cli.path,
            filename: cli.file_name,
            verbose: cli.verbose,
        }
    }

    pub fn file_path(&self) -> PathBuf {
        self.path.join(&self.filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_file_path() {
        let config = Config {
            path: PathBuf::from("/tmp"),
            filename: "test.md".to_string(),
            verbose: false,
        };
        assert_eq!(config.file_path(), PathBuf::from("/tmp/test.md"));
    }

    #[test]
    fn test_config_file_path_current_dir() {
        let config = Config {
            path: PathBuf::from("."),
            filename: "README.md".to_string(),
            verbose: false,
        };
        assert_eq!(config.file_path(), PathBuf::from("./README.md"));
    }
}
