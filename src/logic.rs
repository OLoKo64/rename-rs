use glob::{glob, GlobError, PatternError};
use owo_colors::OwoColorize;
use std::{io::Write, path::Path};

use crate::arg_parser::AppArguments;

pub fn execute(args: &AppArguments, entries: &[std::path::PathBuf]) -> Result<(), std::io::Error> {
    if args.no_act {
        no_act_entries(args, entries, args.start_index);
    } else {
        if args.yes || ask_user("Are you sure you want to rename these entries? (y/N): ")? {
            rename_entries(args, entries, args.start_index)?;

            if !args.no_act {
                println!("\n{}", "All entries were renamed!".green());
            }
        }
        println!("{}", "Aborting rename".yellow());
        return Ok(());
    }

    Ok(())
}

fn no_act_entries(args: &AppArguments, files: &[std::path::PathBuf], replacement: usize) {
    for (i, file) in files.iter().enumerate() {
        println!(
            "Would rename {} to {}",
            file.display().green(),
            new_name(args, file, replacement + i).green()
        );
    }
}

fn new_name(args: &AppArguments, file: &std::path::Path, replacement: usize) -> String {
    let extension = if file.is_dir() {
        file.extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
    } else {
        let extension = file
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        if extension.is_empty() {
            String::new()
        } else {
            format!(".{extension}")
        }
    };
    let file_path = file.parent().unwrap();
    format!(
        "{}-{}{}",
        file_path
            .join(
                args.replacement
                    .as_deref()
                    .unwrap_or(Path::new(file).file_stem().unwrap().to_str().unwrap())
            )
            .to_str()
            .unwrap(),
        replacement,
        extension
    )
}

fn rename_entries(
    args: &AppArguments,
    files: &[std::path::PathBuf],
    replacement: usize,
) -> Result<(), std::io::Error> {
    for (i, file) in files.iter().enumerate() {
        let new_name = new_name(args, file, replacement + i);
        std::fs::rename(file, &new_name)?;
        println!("Renamed {} to {}", file.display().green(), new_name.green());
    }

    Ok(())
}

pub fn only_dirs_entries(paths: Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
    paths.into_iter().filter(|p| p.is_dir()).collect()
}

pub fn only_files_entries(paths: Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
    paths.into_iter().filter(|p| p.is_file()).collect()
}

pub fn all_files_in_dir(pattern: &str) -> Result<Vec<std::path::PathBuf>, GlobErrors> {
    let paths = glob(pattern).map_err(GlobErrors::PatternError)?;

    let path: Result<Vec<_>, _> = paths.collect();
    path.map_err(GlobErrors::GlobError)
}

fn ask_user(question: &str) -> Result<bool, std::io::Error> {
    print!("{question}");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input.to_lowercase().trim() == "y")
}

#[derive(Debug)]
pub enum GlobErrors {
    PatternError(PatternError),
    GlobError(GlobError),
}

impl std::error::Error for GlobErrors {}

impl std::fmt::Display for GlobErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobErrors::PatternError(err) => write!(f, "PatternError: {err}"),
            GlobErrors::GlobError(err) => write!(f, "GlobError: {err}"),
        }
    }
}
