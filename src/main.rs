mod basequal0;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "oodles",
    author,
    version,
    about = "oodles of tools for bioinformatics"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        arg_required_else_help = true,
        about = "set selected base-quality scores in a bam to 0"
    )]
    BaseQual0 {
        #[arg(help = "input bam file")]
        input: String,
        #[arg(help = "output bam file", default_value = "/dev/stdout")]
        output: String,

        #[arg(
            long,
            help = "clip this many bases from left end of read1 (as aligned)",
            default_value_t = 0
        )]
        left_read1: usize,
        #[arg(
            long,
            help = "clip this many bases from right end of read1 (as aligned)",
            default_value_t = 0
        )]
        right_read1: usize,

        #[arg(
            long,
            help = "clip this many bases from left end of read2 (as aligned)",
            default_value_t = 0
        )]
        left_read2: usize,
        #[arg(
            long,
            help = "clip this many bases from right end of read2 (as aligned)",
            default_value_t = 0
        )]
        right_read2: usize,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.command {
        Commands::BaseQual0 {
            input,
            output,
            left_read1,
            right_read1,
            left_read2,
            right_read2,
        } => {
            basequal0::basequal0(
                input,
                output,
                left_read1,
                right_read1,
                left_read2,
                right_read2,
            )?;
        }
    }
    Ok(())
}
