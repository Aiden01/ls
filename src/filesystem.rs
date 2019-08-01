use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::io::Result;
use std::path::{Path, PathBuf};

use crate::app::Options;

#[derive(Debug)]
pub enum FileType {
    File,
    Directory,
}

pub enum Size {
    Kb(u64),
    Bytes(u64),
    Mb(u64),
    Gb(u64),
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Size::Kb(size) => format!("{} Kb", size),
            Size::Bytes(size) => format!("{} bytes", size),
            Size::Mb(size) => format!("{} Mb", size),
            Size::Gb(size) => format!("{} Gb", size),
        };

        write!(f, "{}", &s)
    }
}

#[derive(Debug)]
pub struct File {
    pub name: OsString,
    pub path: PathBuf,
    pub file_type: FileType,
    pub bytes: u64,
    pub readonly: bool,
}

impl File {
    // Returns human-readable filesize
    pub fn readable(&self) -> Size {
        let size = self.bytes;
        match size {
            _ if size >= 1000000000 => Size::Gb(size / 1000000000),
            _ if size >= 1000000 => Size::Mb(size / 1000000),
            _ if size >= 1000 => Size::Kb(size / 1000),
            _ => Size::Bytes(size),
        }
    }
}

// Returns the information of the file
pub fn get_file_info(file: Result<DirEntry>, ctx: &Options) -> Result<File> {
    let file = file?;
    let file_type = if file.file_type()?.is_dir() {
        FileType::Directory
    } else {
        FileType::File
    };

    let metadata = file.metadata()?;

    let name = file.file_name();
    let path = file.path();
    let bytes = metadata.len();
    let readonly = metadata.permissions().readonly();

    Ok(File {
        name,
        path,
        file_type,
        bytes,
        readonly,
    })
}

pub fn is_hidden(file: &DirEntry) -> bool {
    file.file_name().into_string().unwrap().starts_with('.')
}

pub fn read_dir(path: &Path, ctx: &Options) -> Result<Vec<File>> {
    let Options { all: hidden, .. } = ctx;
    let paths = fs::read_dir(path)?;
    paths
        .filter(|file| *hidden || !is_hidden(file.as_ref().unwrap()))
        .map(|file| get_file_info(file, ctx))
        .collect()
}
