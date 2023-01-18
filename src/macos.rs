#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::fs::File;

use crate::{FileSystemKind, FileSystemName};

pub fn filesystem(file: &File) -> Result<FileSystemKind, std::io::Error> {
    todo!()
}
