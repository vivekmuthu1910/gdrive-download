use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
mod item_macros;
mod utils;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "GDrive-Download",
    about = "A CLI tool to download publicaly shared files from Google Drive.",
    version = "0.1"
)]
pub struct Config {
    /// URL of the file/folder to download
    url: Vec<String>,

    /// Name of the file to saved. If not given, finds file name from Google Drive file.
    #[structopt(short, long)]
    out_file: Option<PathBuf>,

    /// Directory in which the file have to be saved. If not given saves in the current directory.
    #[structopt(short, long)]
    directory: Option<PathBuf>,
}

#[cfg(test)]
mod tests {}
