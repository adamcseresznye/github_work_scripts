mod file_finder;
mod parser;
mod pipeline;

use clap::{command, Arg, ArgMatches};

use crate::pipeline::run;
use anyhow::Result;

fn main() -> Result<()> {
    let input = get_input();

    match run(&input) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
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
