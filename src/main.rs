mod arg_parser;
mod logic;

use clap::Parser;
use logic::{all_files_in_dir, execute, only_dirs_entries, only_files_entries};
use owo_colors::OwoColorize;

use crate::arg_parser::AppArguments;

fn main() {
    let args = AppArguments::parse();

    run(&args).unwrap();
}

fn run(args: &AppArguments) -> Result<(), Box<dyn std::error::Error>> {
    let all_entries = all_files_in_dir(&args.pattern)?;
    if all_entries.is_empty() {
        println!("No entries found for pattern {}", args.pattern.yellow());
        return Ok(());
    }

    if args.files_only {
        let files = only_files_entries(all_entries);
        execute(args, &files)?;
    } else if args.dirs_only {
        let dirs = only_dirs_entries(all_entries);
        execute(args, &dirs)?;
    } else {
        execute(args, &all_entries)?;
    }

    Ok(())
}
