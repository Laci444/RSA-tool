use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "RSA Tool")]
#[command(author = "Laszlo Szilagyi")]
#[command(version = "1.0")]
#[command(about = "Encrypt, decrypt, sign, and verify using RSA", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Keygen {
        #[arg(short = 'p', long)]
        pubkey: PathBuf,

        #[arg(short = 's', long)]
        privkey: PathBuf,

        #[arg(short, long, default_value_t = 2048)]
        bits: u32,
    },
    Encrypt {
        #[arg(short, long)]
        pubkey: PathBuf,

        #[arg(short = 'i', long)]
        r#in: Option<PathBuf>,

        #[arg(short = 'o', long)]
        out: Option<PathBuf>,
    },
    Decrypt {
        #[arg(short, long)]
        privkey: PathBuf,

        #[arg(short = 'i', long)]
        r#in: Option<PathBuf>,

        #[arg(short = 'o', long)]
        out: Option<PathBuf>,
    },
    Sign {
        #[arg(short, long)]
        privkey: PathBuf,

        #[arg(short = 'i', long)]
        r#in: Option<PathBuf>,

        #[arg(short = 'o', long)]
        out: Option<PathBuf>,
    },
    Verify {
        #[arg(short, long)]
        pubkey: PathBuf,

        #[arg(short = 'i', long)]
        r#in: Option<PathBuf>,

        #[arg(short, long)]
        sig: PathBuf,
    },
}

