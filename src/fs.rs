// code taken from https://github.com/rust-lang/mdBook
// SPDX-License-Identifier: MPL-2.0

use std::convert::Into;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use log::{debug, trace};
use mdbook::errors::Result;

/// Write the given data to a file, creating it first if necessary
pub fn write_file<P: AsRef<Path>>(build_dir: &Path, filename: P, content: &[u8]) -> Result<()> {
    let path = build_dir.join(filename);

    create_file(&path)?.write_all(content).map_err(Into::into)
}

/// This function creates a file and returns it. But before creating the file
/// it checks every directory in the path to see if it exists,
/// and if it does not it will be created.
pub fn create_file(path: &Path) -> Result<File> {
    debug!("Creating {}", path.display());

    // Construct path
    if let Some(p) = path.parent() {
        trace!("Parent directory is: {:?}", p);

        fs::create_dir_all(p)?;
    }

    File::create(path).map_err(Into::into)
}
