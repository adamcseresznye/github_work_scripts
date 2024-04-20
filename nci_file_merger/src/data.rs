use anyhow::anyhow;
use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct FileFinder {
    pub file_locations: Vec<String>,
    pub sample_names: Vec<String>,
}

impl FileFinder {
    pub fn new() -> Self {
        Self {
            file_locations: Vec::new(),
            sample_names: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct ParseConfig {
    pub column_starts: usize,
    pub column_width: usize,
    pub rows_to_skip_beginning: usize,
    pub rows_to_take: usize,
}

fn is_target(entry: &DirEntry, filename: &str) -> bool {
    entry.file_name().to_string_lossy().ends_with(filename)
}

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
