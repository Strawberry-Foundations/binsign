use clap::{Parser, Subcommand};

mod gen;
mod sign;
mod check;

/// Simple utility for signing binaries
#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a new certificate and save it as a file
    Gen {
        /// Output location for the certificate
        #[clap(short, long)]
        out: String,
    },
    /// Sign a binary file
    Sign {
        /// File to be signed
        #[clap(short, long)]
        file: String,

        /// Key to be used
        #[clap(short, long)]
        key: String,
    },
    /// Compare a file with a certificate
    Check {
        /// File to be checked
        #[clap(short, long)]
        file: String,

        /// Key to be used
        #[clap(short, long)]
        key: String,
    }
}

#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "matteodev8")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen { out } => gen::gen_cmd(&*out),
        Commands::Sign { file, key } => sign::sign_cmd(&*file, &*key),
        Commands::Check { file, key } => check::check_cmd(&*file, &*key),
    }
}
