//! The `parser` module provides functionality for parsing fixed-width data.
//!
//! It includes the `ParseConfig` struct, which holds the configuration for parsing,
//! and two functions: `parse_fixed_width` and `create_parse_config`.
//!
//! The `parse_fixed_width` function takes a string of data and a `ParseConfig` as arguments,
//! and returns a vector of strings, each representing a field from the fixed-width data.
//!
//! The `create_parse_config` function takes command line arguments and creates a `ParseConfig` from them.
//! It expects the arguments to include `column_starts`, `column_width`, `rows_to_skip_beginning`, and `rows_to_take`.
//! Each of these arguments is expected to be a string that can be parsed into a `usize`.
//!
//! # Examples
//!
//! ```
//! use crate::parser::{ParseConfig, parse_fixed_width, create_parse_config};
//! use clap::ArgMatches;
//!
//! let input = ArgMatches::new(); // Assume this is filled with actual arguments
//! let config = create_parse_config(&input, "column_starts", "column_width").unwrap();
//! let data = "some fixed-width data";
//! let parsed_data = parse_fixed_width(data, config).unwrap();
//! ```
//!
//! This module is part of a larger application that processes fixed-width data.

use anyhow::anyhow;
use anyhow::Result;
use clap::ArgMatches;

/// The `ParseConfig` struct is used to hold the configuration for parsing fixed-width data.
/// It includes the starting column, column width, number of rows to skip at the beginning, and the number of rows to take.
#[derive(Debug)]
pub struct ParseConfig {
    pub column_starts: usize,
    pub column_width: usize,
    pub rows_to_skip_beginning: usize,
    pub rows_to_take: usize,
}

/// The `parse_fixed_width` function takes a string of data and a `ParseConfig` as arguments.
/// It returns a vector of strings, each representing a field from the fixed-width data.
/// The function skips the specified number of rows at the beginning, then takes the specified number of rows.
/// For each row, it extracts the field starting at the specified column and of the specified width.
/// The extracted field is trimmed and added to the result vector.
pub fn parse_fixed_width(
    data: &str,
    parse_config: ParseConfig,
) -> Result<Vec<String>, anyhow::Error> {
    Ok(data
        .lines()
        .skip(parse_config.rows_to_skip_beginning)
        .take(parse_config.rows_to_take)
        .filter_map(|line| {
            line.get(
                parse_config.column_starts..parse_config.column_starts + parse_config.column_width,
            )
        })
        .map(|field| field.trim().to_owned())
        .collect())
}

/// The `create_parse_config` function takes the command line arguments and creates a `ParseConfig` from them.
/// It expects the arguments to include `column_starts`, `column_width`, `rows_to_skip_beginning`, and `rows_to_take`.
/// Each of these arguments is expected to be a string that can be parsed into a `usize`.
/// If any argument is missing or cannot be parsed into a `usize`, an error is returned.
pub fn create_parse_config(
    input: &ArgMatches,
    starts: &str,
    width: &str,
) -> Result<ParseConfig, anyhow::Error> {
    let column_starts = input
        .get_one::<String>(starts)
        .ok_or(anyhow!("column_starts argument not found"))?
        .parse::<usize>()
        .map_err(|_| anyhow!("Failed to parse the column_starts argument as usize"))?;

    let column_width = input
        .get_one::<String>(width)
        .ok_or(anyhow!("column_width argument not found"))?
        .parse::<usize>()
        .map_err(|_| anyhow!("Failed to parse column_width argument as usize"))?;

    let rows_to_skip_beginning = input
        .get_one::<String>("rows_to_skip_beginning")
        .ok_or(anyhow!("rows_to_skip_beginning argument not found"))?
        .parse::<usize>()
        .map_err(|_| anyhow!("Failed to parse the rows_to_skip_beginning argument as usize"))?;

    let rows_to_take = input
        .get_one::<String>("rows_to_take")
        .ok_or(anyhow!("rows_to_take argument not found"))?
        .parse::<usize>()
        .map_err(|_| anyhow!("Failed to parse rows_to_take argument as usize"))?;

    Ok(ParseConfig {
        column_starts,
        column_width,
        rows_to_skip_beginning,
        rows_to_take,
    })
}
