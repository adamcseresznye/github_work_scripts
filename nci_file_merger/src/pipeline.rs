//! The `pipeline` module provides functionality for processing files.
//!
//! It includes several functions: `run`, `parse_argument`, `process_files`, `create_dataframe`, and `save_dataframes`.
//!
//! The `run` function is the main function that orchestrates the processing of files. It takes command line arguments, finds files, processes them, and optionally saves the results.
//!
//! The `parse_argument` function is a helper function that parses a command line argument and returns an error if the argument is not provided.
//!
//! The `process_files` function processes the files found by the `FileFinder`. It creates dataframes for response and concentration data, and filters out any rows to drop.
//!
//! The `create_dataframe` function creates a dataframe from the contents of a file. It uses the `parser` module to parse the fixed-width data.
//!
//! The `save_dataframes` function saves the dataframes to CSV files. It takes a path and a vector of dataframes as arguments.
//!
//! # Examples
//!
//! ```
//! use crate::pipeline::{run, parse_argument, process_files, create_dataframe, save_dataframes};
//! use clap::ArgMatches;
//!
//! let input = ArgMatches::new(); // Assume this is filled with actual arguments
//! run(&input).unwrap();
//! ```
//!
//! This module is part of a larger application that processes data from files.

use crate::file_finder;
use crate::file_finder::FileFinder;
use crate::parser;

use anyhow::anyhow;
use anyhow::Result;
use clap::ArgMatches;
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

use std::fs;

/// The `run` function is the main function that orchestrates the processing of files.
/// It takes command line arguments, finds files, processes them, and optionally saves the results.
pub fn run(input: &ArgMatches) -> Result<()> {
    let path = parse_argument(
        input,
        "path",
        "The required 'path' argument was not provided",
    )?;
    let file = parse_argument(
        input,
        "file",
        "The required 'file' argument was not provided",
    )?;
    let save = parse_argument(input, "save", "The 'save' argument was not provided")?
        .parse::<bool>()
        .map_err(|_| anyhow!("Failed to parse save argument as bool"))?;

    let row_to_drop: Option<usize> = input
        .get_one::<String>("row_to_drop")
        .and_then(|s| s.parse::<usize>().ok());

    let files = file_finder::find_files(path.as_str(), file.as_str())?;

    let (response_df, concentration_df) = process_files(&files, input, row_to_drop)?;

    if save {
        save_dataframes(path.as_str(), vec![response_df, concentration_df])?;
        println!("\nSuccess! Files were saved at {}.", path);
    } else {
        println!(
            "\nSuccess! Your files have been extracted. Please check for potential formatting errors. When you're ready to save your files to a CSV, simply apply the --save true command."
        );
        println!("\nExtracted peak areas: {:?}", response_df);
        println!("\nExtracted concentrations: {:?}", concentration_df);
    }
    Ok(())
}

/// The `parse_argument` function is a helper function that parses a command line argument and returns an error if the argument is not provided.
fn parse_argument(input: &ArgMatches, arg: &str, error_msg: &str) -> Result<String> {
    input
        .get_one::<String>(arg)
        .ok_or(anyhow!(error_msg.to_string()))
        .cloned()
}

/// The `process_files` function processes the files found by the `FileFinder`.
/// It creates dataframes for response and concentration data, and filters out any rows to drop.
fn process_files(
    files: &FileFinder,
    input: &ArgMatches,
    row_to_drop: Option<usize>,
) -> Result<(DataFrame, DataFrame)> {
    let mut response_df = DataFrame::default();
    let mut concentration_df = DataFrame::default();
    for (count, (location, name)) in files
        .file_locations
        .iter()
        .zip(files.sample_names.iter())
        .enumerate()
    {
        let contents = fs::read_to_string(location)?;

        if count == 0 {
            let names =
                create_dataframe(input, "compound", &contents, "name_starts", "name_width")?;
            let names_clone = names.clone();
            response_df.with_column(names)?;
            concentration_df.with_column(names_clone.clone())?;
        }

        let response_series = create_dataframe(
            input,
            name.as_str(),
            &contents,
            "response_starts",
            "response_width",
        )?;
        let concentration_series =
            create_dataframe(input, name.as_str(), &contents, "conc_starts", "conc_width")?;

        response_df.with_column(response_series)?;
        concentration_df.with_column(concentration_series)?;
    }
    if let Some(row_to_drop) = row_to_drop {
        let mask = (0..response_df.height())
            .map(|i| i != row_to_drop)
            .collect();
        response_df = response_df.filter(&mask)?;
        concentration_df = concentration_df.filter(&mask)?;
    }
    Ok((response_df, concentration_df))
}

/// The `create_dataframe` function creates a dataframe from the contents of a file.
/// It uses the `parser` module to parse the fixed-width data.
fn create_dataframe(
    input: &ArgMatches,
    name: &str,
    contents: &str,
    config_starts: &str,
    config_width: &str,
) -> Result<Series, anyhow::Error> {
    let config = parser::create_parse_config(input, config_starts, config_width)?;
    let parsed_data = parser::parse_fixed_width(contents, config)?;
    Ok(Series::new(name, parsed_data))
}

/// The `save_dataframes` function saves the dataframes to CSV files.
/// It takes a path and a vector of dataframes as arguments.
fn save_dataframes(path: &str, dataframes: Vec<DataFrame>) -> Result<()> {
    for (file_name, df) in ["peak_areas.csv", "concentrations.csv"]
        .iter()
        .zip(vec![&dataframes[0], &dataframes[1]])
    {
        let full_path = Path::new(&path).join(file_name);
        let mut file = File::create(full_path).expect("could not create file");

        let mut df = df.clone();

        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)?
    }
    Ok(())
}
