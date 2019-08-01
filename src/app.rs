use std::io::Error as IOError;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::filesystem::read_dir;
use crate::grid::{display_files};

/// ls clone written in Rust
#[derive(Debug, StructOpt)]
#[structopt(name = "ls")]
pub struct Options {
    /// Whether to display hidden files
    #[structopt(short = "a", long = "all")]
    pub all: bool,

    /// Path to directory
    #[structopt(name = "DIRECTORY", parse(from_os_str))]
    pub dir: PathBuf,

    /// Whether to display human-readable file sizes
    #[structopt(short = "h", long = "human")]
    pub human: bool,
}

pub struct App<'a> {
    ctx: &'a Options,
}

impl<'a> App<'a> {
    pub fn new(ctx: &'a Options) -> Self {
        App { ctx }
    }

    pub fn run(&self) -> Result<(), IOError> {
        let Options { dir, all, .. } = self.ctx;
        let files = read_dir(dir, self.ctx)?;
        let grid = display_files(files, self.ctx);
        println!("{}", grid.fit_into_width(54).unwrap());
        Ok(())
    }
}
