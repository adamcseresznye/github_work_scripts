mod file_finder;
mod parser;

use crate::file_finder::find_files;
use crate::parser::parse_fixed_width;

use clap::{command, Arg, ArgMatches};

use crate::parser::create_parse_config;
use anyhow::anyhow;
use anyhow::Result;
use csv::Writer;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    let input = get_input();

    let path = input
        .get_one::<String>("path")
        .expect("Invalid path provided.")
        .as_str();
    let input_clone = input.clone();
    match run(input_clone) {
        Ok(_) => println!("Success! Files were saved at {}.", path),
        Err(e) => println!("An error occurred: {}", e),
    }
}

fn run(input: ArgMatches) -> Result<()> {
    let path = input
        .get_one::<String>("path")
        .ok_or(anyhow!("Path argument not found"))?
        .as_str();
    let file = input
        .get_one::<String>("file")
        .ok_or(anyhow!("File argument not found"))?
        .as_str();
    let save = input
        .get_one::<String>("save")
        .ok_or(anyhow!("Save argument not found"))?
        .as_str()
        .parse::<bool>()
        .map_err(|_| anyhow!("Failed to parse save argument as bool"))?;

    let files = find_files(path, file)?;

    let mut names_to_save = Vec::new();
    let mut responses_to_save = Vec::new();
    let mut conc_to_save = Vec::new();

    for (location, name) in files.file_locations.iter().zip(files.sample_names.iter()) {
        let contents = fs::read_to_string(location)?;

        let name_config = create_parse_config(&input, "name_starts", "name_width")?;
        let conc_config = create_parse_config(&input, "conc_starts", "conc_width")?;
        let response_config = create_parse_config(&input, "response_starts", "response_width")?;

        let mut names = parse_fixed_width(contents.as_str(), name_config);
        let mut responses = parse_fixed_width(contents.as_str(), response_config);
        let mut concentrations = parse_fixed_width(contents.as_str(), conc_config);

        names.insert(0, "file_name".to_string());
        responses.insert(0, name.to_string());
        concentrations.insert(0, name.to_string());

        names_to_save.push(names);
        responses_to_save.push(responses);
        conc_to_save.push(concentrations);
    }

    if save {
        let responses_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(Path::new(path).join("responses.csv"))?;
        let concentrations_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(Path::new(path).join("concentrations.csv"))?;

        let mut responses_wtr = Writer::from_writer(responses_file);
        let mut concentrations_wtr = Writer::from_writer(concentrations_file);

        for (idx, row) in responses_to_save.iter().enumerate() {
            if idx == 0 {
                responses_wtr.write_record(&names_to_save[0])?;
            }
            responses_wtr.write_record(row)?;
        }
        for (idx, row) in conc_to_save.iter().enumerate() {
            if idx == 0 {
                concentrations_wtr.write_record(&names_to_save[0])?;
            }
            concentrations_wtr.write_record(row)?;
        }
        responses_wtr.flush()?;
        concentrations_wtr.flush()?;
    }

    Ok(())
}

fn get_input() -> ArgMatches {
    let input = command!()
    .about("A command-line utility designed to extract concentration and peak area data from files generated on the Agilent 5973 (NCI).")
        .arg(
            Arg::new("path")
        .long("path")
        .short('p')
        .required(true)
        .help(
            r"The root directory of the project. For instance, C:\Users\myname\myproject.",
        ))
        .arg(
            Arg::new("file")
        .long("file")
        .short('f')
        .default_value("a-all.txt")
        .help(
            r"The common file name that contains the peak areas and concentration data. Default is 'a-all.txt'.",
        )
        )
        .arg(
            Arg::new("save")
        .long("save")
        .help(
            r"Determines whether the files should be saved. Default is true.",
        )
        .default_value("true")
        )
        .arg(
            Arg::new("name_starts")
        .long("name_starts")
        .default_value("8")
        .help(
            r"The starting position of the name column. Default is 8.",
        )
        )
        .arg(
            Arg::new("name_width")
        .long("name_width")
        .default_value("20")
        .help(
            r"The width of the name column. Default is 20.",
        )
        )
        .arg(
            Arg::new("response_starts")
        .long("response_starts")
        .default_value("49")
        .help(
            r"The starting position of the response column. Default is 49.",
        )
        )
        .arg(
            Arg::new("response_width")
        .long("response_width")
        .default_value("6")
        .help(
            r"The width of the response column. Default is 6.",
        )
        )
        .arg(
            Arg::new("conc_starts")
        .long("conc_starts")
        .default_value("56")
        .help(
            r"The starting position of the concentration column. Default is 56.",
        )
        )
        .arg(
            Arg::new("conc_width")
        .long("conc_width")
        .default_value("9")
        .help(
            r"The width of the concentration column. Default is 9.",
        )
        )
        .arg(
            Arg::new("rows_to_skip_beginning")
        .long("rows_to_skip_beginning")
        .default_value("19")
        .help(
            r"The number of rows to skip at the beginning of the file. Default is 19.",
        )
        )
        .arg(
            Arg::new("rows_to_take")
        .long("rows_to_take")
        .default_value("25")
        .help(
            r"The number of rows to process from the top of the file. Default is 25.",
        )
    )
        .get_matches();

    input
}

// 339370
