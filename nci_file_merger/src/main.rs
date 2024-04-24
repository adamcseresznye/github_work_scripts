mod file_finder;
mod parser;

use crate::file_finder::find_files;
use crate::parser::parse_fixed_width;

use clap::{command, Arg, ArgMatches};

use crate::parser::create_parse_config;
use anyhow::anyhow;
use anyhow::Result;
use polars::prelude::*;
use std::fs::File;
use std::path::Path;


use std::fs;

fn main() -> Result<()> {
    let input = get_input();

    let path = input
        .get_one::<String>("path")
        .ok_or(anyhow!("Path argument not found"))?
        .as_str();

    let save: bool = input
        .get_one::<String>("save")
        .ok_or(anyhow!("Save argument not found."))?
        .parse()?;

    match run(&input) {
        Ok(_) if save => {
            println!("\nSuccess! Files were saved at {}.", path);
            Ok(())
        }
        Ok(_) => {
            println!(
                "\nSuccess! Your files have been successfully extracted. When you're prepared to save your files to a CSV, simply apply the --save true command."
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn run(input: &ArgMatches) -> Result<()> {
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
        .parse::<bool>()
        .map_err(|_| anyhow!("Failed to parse save argument as bool"))?;

        let row_to_drop: Option<usize> = input
        .get_one::<String>("row_to_drop")
        .and_then(|s| s.parse::<usize>().ok());


    let files = find_files(path, file)?;

    let mut response_df = DataFrame::default();
    let mut concentration_df = DataFrame::default();
    for (count, (location, name)) in files.file_locations.iter().zip(files.sample_names.iter()).enumerate() {

        let contents = fs::read_to_string(location)?;

        if count == 0 {
            let name_config = create_parse_config(input, "name_starts", "name_width")?;
            let names = parse_fixed_width(contents.as_str(), name_config)?;
            let names_clone = names.clone();

            response_df.with_column(Series::new("compound", names))?;
            concentration_df.with_column(Series::new("compound", names_clone))?;

        }

        let conc_config = create_parse_config(input, "conc_starts", "conc_width")?;
        let response_config = create_parse_config(input, "response_starts", "response_width")?;

        let responses = parse_fixed_width(contents.as_str(), response_config);
        let concentrations = parse_fixed_width(contents.as_str(), conc_config);

        response_df.with_column(Series::new(format!("{}", name).as_str(), responses?))?;
        concentration_df.with_column(Series::new(format!("{}", name).as_str(), concentrations?))?;

    }
    if let Some(row_to_drop) = row_to_drop {
        let mask = (0..response_df.height()).map(|i| i != row_to_drop).collect();
        response_df = response_df.filter(&mask)?;
        concentration_df = concentration_df.filter(&mask)?;
    }

    if save {
        for (file_name, mut df) in ["peak_areas.csv", "concentrations.csv"]
            .iter()
            .zip(vec![response_df, concentration_df])

        {   let full_path = Path::new(path).join(file_name);
            let mut file = File::create(full_path).expect("could not create file");

        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)?
        }
    } else {
        println!("Extracted peak areas:");
        println!("{:?}", response_df);
        println!("\nExtracted concentrations:");
        println!("{:?}", concentration_df);
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
            r"Set to true to save files to CSV. Set to false to display files on the command line. Default is true.",
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
    .arg(
        Arg::new("row_to_drop")
    .long("row_to_drop")
    .required(false)
    .help(
        r"The row ID to exclude from the dataframe. For instance, 3.",
    )

)
        .get_matches();
    input
}
