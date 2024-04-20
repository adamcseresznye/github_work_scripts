use anyhow::anyhow;
use anyhow::Result;
use clap::ArgMatches;

#[derive(Debug)]
pub struct ParseConfig {
    pub column_starts: usize,
    pub column_width: usize,
    pub rows_to_skip_beginning: usize,
    pub rows_to_take: usize,
}

pub fn parse_fixed_width(data: &str, parse_config: ParseConfig) -> Vec<String> {
    data.lines()
        .skip(parse_config.rows_to_skip_beginning)
        .take(parse_config.rows_to_take)
        .filter_map(|line| {
            line.get(
                parse_config.column_starts..parse_config.column_starts + parse_config.column_width,
            )
        })
        .map(|field| field.trim().to_owned())
        .collect()
}

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
