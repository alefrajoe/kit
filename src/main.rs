use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about="Count the number of files in a directory")]
    Count {
        #[arg(short, long, help="Path to the file or directory")]
        path: String,
        #[arg(short, long, help="Pattern to match the files")]
        pattern: Option<String>,
    },
    #[command(about="Calculate the checksum of an image file or directory")]
    Imagesum {
        #[arg(short='p', long="path", help="Path to the image file or directory")]
        path: String,
    },
}

fn main() {
    // Parse the command line arguments
    let cli = Cli::parse();
    // Match the command
    match cli.command {
        Command::Count { path, pattern } => {
            commands::count::count_files(&path, pattern);
        },
        Command::Imagesum { path } => {
            commands::imagesum::count_images_with_size(&path);
        },
    }
}