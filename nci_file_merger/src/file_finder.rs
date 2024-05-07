//! The `file_finder` module provides functionality for finding files in a directory.
//!
//! It includes the `FileFinder` struct, which holds the locations of the found files and their sample names,
//! and two functions: `is_target` and `find_files`.
//!
//! The `FileFinder` struct is used to store the locations of the found files and their sample names.
//! It has a constructor `new` that initializes an empty `FileFinder`.
//!
//! The `is_target` function is a helper function that checks if a directory entry ends with a given filename.
//!
//! The `find_files` function takes a path and a filename as arguments, and returns a `FileFinder` with the locations of the found files and their sample names.
//! It walks through the directory specified by the path, and for each entry, it checks if the entry is a target file using the `is_target` function.
//! If the entry is a target file, its location is added to the `file_locations` of the `FileFinder`, and its parent folder name is added to the `sample_names` of the `FileFinder`.
//! If the parent folder name cannot be found, an error is returned.
//!
//! # Examples
//!
//! ```
//! use crate::file_finder::{FileFinder, find_files};
//!
//! let path = "some/directory";
//! let filename = "target.txt";
//! let file_finder = find_files(path, filename).unwrap();
//! ```
//!
//! This module is part of a larger application that processes data from files.
use anyhow::anyhow;
use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

/// The `FileFinder` struct is used to store the locations of the found files and their sample names.
///
/// # Fields
///
/// * `file_locations` - A vector of strings, where each string is the location of a found file.
/// * `sample_names` - A vector of strings, where each string is the sample name of a found file.

#[derive(Debug)]
pub struct FileFinder {
    pub file_locations: Vec<String>,
    pub sample_names: Vec<String>,
}
/// Implementation of the `FileFinder` struct.
impl FileFinder {
    /// Constructs a new `FileFinder` with empty `file_locations` and `sample_names`.
    ///
    /// # Returns
    ///
    /// * `FileFinder` - A new `FileFinder` with empty `file_locations` and `sample_names`.
    pub fn new() -> Self {
        Self {
            file_locations: Vec::new(),
            sample_names: Vec::new(),
        }
    }
}
/// Checks if a directory entry is a target file.
///
/// # Arguments
///
/// * `entry` - A reference to a directory entry.
/// * `filename` - The name of the target file.
///
/// # Returns
///
/// * `bool` - Returns true if the directory entry is a target file, false otherwise.
fn is_target(entry: &DirEntry, filename: &str) -> bool {
    entry.file_name().to_string_lossy().ends_with(filename)
}
/// Finds files in a directory.
///
/// # Arguments
///
/// * `path` - The path of the directory to search in.
/// * `filename` - The name of the target file.
///
/// # Returns
///
/// * `Result<FileFinder>` - Returns a `FileFinder` with the locations of the found files and their sample names, or an error if the parent folder name cannot be found.
pub fn find_files(path: &str, filename: &str) -> Result<FileFinder> {
    let mut file_finder: FileFinder = FileFinder::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_target(e, filename))
    {
        let path = entry.path();
        file_finder.file_locations.push(path.display().to_string());
        let parent_folder = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());
        if let Some(name) = parent_folder {
            file_finder.sample_names.push(name.to_string());
        } else {
            return Err(anyhow!("Parent folder not found"));
        }
    }
    Ok(file_finder)
}
