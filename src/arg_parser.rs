use clap::Parser;

/// A simple rename tool
#[derive(Parser)]
#[command(version)]
pub struct AppArguments {
    /// The pattern to search for
    pub pattern: String,

    /// Do not rename, just print what would be renamed
    #[arg(short, long)]
    pub no_act: bool,

    /// Only rename files, not directories
    #[arg(short, long, group = "file_or_dir")]
    pub files_only: bool,

    /// Only rename directories, not files
    #[arg(short, long, group = "file_or_dir")]
    pub dirs_only: bool,

    /// The replacement starting range to use
    #[arg(short = 'i', long, default_value_t = 1)]
    pub start_index: usize,

    /// The replacement file name
    #[arg(short, long)]
    pub replacement: Option<String>,

    /// Bypass the confirmation prompt and rename all entries
    #[arg(short, long)]
    pub yes: bool,

    /// Index separator
    #[arg(short, long, default_value = "-")]
    pub separator: String,
}
