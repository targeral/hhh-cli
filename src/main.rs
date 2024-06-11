// use indicatif::{ProgressBar, ProgressStyle};
use clap::{Parser, Subcommand};
// use anyhow::{Context, Result};
// use std::fs::File;
// use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
}



fn main() {
    let cli = Cli::parse();
    let context = hhh::init_command_context();

    match &cli.command {
        Commands::Init => {
            hhh::commands_init(context);
        }
    }

    // let file = File::open(&args.path)
    //     .with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    // let meta = file.metadata()?;
    // let progress_bar = ProgressBar::new(meta.len());
    // progress_bar.set_style(ProgressStyle::default_bar()
    //     .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    //     ?.progress_chars("#>-"));
    // let reader = BufReader::new(file);
    // let mut results = vec![];
    // for line in reader.lines() {
    //     let line_content: String = line?;

    //     if line_content.contains(&args.pattern) {
    //         results.push(line_content.clone());
    //     }
    //     progress_bar.inc(line_content.len() as u64 + 1);
    // }
    // progress_bar.finish_with_message("done");

    // if results.len() > 0 {
    //     for result in results {
    //         println!("{result}")
    //     }
    // }
    // Ok(())
}
