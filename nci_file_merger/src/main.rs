use glob::glob;
use polars::prelude::*;
use std::path::Path;

struct FileFinder {
    file_locations: Vec<String>,
    sample_names: Vec<String>,
}

impl FileFinder {
    fn new() -> Self {
        Self {
            file_locations: Vec::new(),
            sample_names: Vec::new(),
        }
    }
}

struct DataExtractor {
    response_series: Vec<Series>,
    concentration_series: Vec<Series>,
    compound_names: Vec<Series>,
}

impl DataExtractor {
    fn new() -> Self {
        Self {
            response_series: Vec::new(),
            concentration_series: Vec::new(),
            compound_names: Vec::new(),
        }
    }
}

struct ParseConfig {
    column_starts: [i64; 6],
    column_lengths: [i64; 6],
    rows_to_read_in: usize,
    rows_to_skip_beginning: usize,
    rows_to_drop_end: usize,
}

fn find_files(path: &str, filename: &str) -> Result<FileFinder, String> {
    let pattern = Path::new(path).join(format!("**/{}", filename));
    let mut file_finder: FileFinder = FileFinder::new();
    for file in glob(pattern.to_str().unwrap()).map_err(|e| e.to_string())? {
        match file {
            Ok(path) => {
                file_finder.file_locations.push(path.display().to_string());
                let parent_folder = path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str());
                if let Some(name) = parent_folder {
                    file_finder.sample_names.push(name.to_string());
                } else {
                    return Err(String::from("Failed to get parent folder name"));
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
    Ok(file_finder)
}

fn parse_fwd_txt(config: &ParseConfig, path: &str) -> Result<DataFrame, String> {
    // Specify columns positions and lengths, names and types of columns to parse
    let column_names: Vec<&str> = vec![
        "number",
        "compound",
        "Rt",
        "Qion",
        "Response",
        "Concentration",
    ];

    // initialize a vector to "list" polars expressions, on per column
    let mut _vec_expr: Vec<Expr> = vec![];

    let mut _i = 0;

    while _i < column_names.len() {
        _vec_expr.append(&mut vec![col("l")
            .str()
            .slice(
                lit(config.column_starts[_i]),
                lit::<i64>(config.column_lengths[_i].try_into().unwrap()),
            )
            .alias(column_names[_i])]);
        _i += 1;
    }

    // Read with csv reader lazily (if you have comma in the file, change the delimiter)
    let mut data_ = LazyCsvReader::new(path)
        // read just one column named "l" for line
        .with_schema(Some(Arc::new(
            Schema::from_iter(vec![Field::new(
                "l",
                DataType::from_arrow(&ArrowDataType::Utf8, true),
            )])
            .into(),
        )))
        .has_header(false)
        // test 100 first lines
        .with_n_rows(Some(config.rows_to_read_in))
        .with_skip_rows(config.rows_to_skip_beginning)
        .finish()
        .map_err(|e| e.to_string())?; // Add error handling with ?

    // append the polars lazyframe with the expressions generated above
    data_ = data_.with_columns(_vec_expr);

    // collect
    let mut df = data_.collect().map_err(|e| e.to_string())?; // Add error handling with ?

    // Get the number of rows in the DataFrame
    let nrows = df.height();

    // Drop the last 5 rows
    if nrows > config.rows_to_drop_end {
        df = df.slice(0, nrows - config.rows_to_drop_end);
    }

    Ok(df)
}

fn get_dataframes(
    found_files: FileFinder,
    config: ParseConfig,
) -> Result<(DataFrame, DataFrame), String> {
    let mut extracted_files = DataExtractor::new();
    for (location, name) in found_files
        .file_locations
        .iter()
        .zip(found_files.sample_names)
    {
        let temp_df = parse_fwd_txt(&config, &location)?;
        let response = temp_df
            .column("Response")
            .expect("Failed to select column.")
            .clone()
            .rename(&name)
            .clone();
        let concentration = temp_df
            .column("Concentration")
            .expect("Failed to select column.")
            .clone()
            .rename(&name)
            .clone();
        let compound_name = temp_df.column("compound").expect("REASON").clone();
        extracted_files.compound_names.push(compound_name);
        extracted_files.response_series.push(response);
        extracted_files.concentration_series.push(concentration);
    }

    // Create a new DataFrame with the first series in compound_names
    let compound_df = DataFrame::new(vec![extracted_files.compound_names.remove(0)])
        .expect("Failed to create DataFrame");

    let response_df =
        DataFrame::new(extracted_files.response_series).expect("Failed to create DataFrame");
    let concentration_df =
        DataFrame::new(extracted_files.concentration_series).expect("Failed to create DataFrame");

    // Concatenate response_df onto compound_df
    let result_response_df = compound_df
        .hstack(response_df.get_columns())
        .expect("Failed to concatenate DataFrames");
    let result_concentration_df = compound_df
        .hstack(concentration_df.get_columns())
        .expect("Failed to concatenate DataFrames");
    Ok((result_response_df, result_concentration_df))
}

fn main() -> Result<(), String> {
    let files = find_files(
        r"C:\Users\s0212777\OneDrive - Universiteit Antwerpen\Work\github_work_scripts\nci_file_merger\data",
        "a-all.txt",
    )?;
    let parse_config = ParseConfig {
        column_starts: [0, 7, 20, 41, 47, 56],
        column_lengths: [6, 4, 5, 25, 8, 8],
        rows_to_read_in: 100,
        rows_to_skip_beginning: 19,
        rows_to_drop_end: 5,
    };

    let extracted_dfs = get_dataframes(files, parse_config)?;
    println!("{:?}, {:?}", extracted_dfs.0, extracted_dfs.1);
    Ok(())
}
