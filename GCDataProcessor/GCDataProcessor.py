from pathlib import Path

import click
import pandas as pd


def process_and_export_GC_data(path, index_to_drop, export=True):
    """
    Reads in data from text files in a specified directory, processes the data,
    and optionally exports the results to CSV files.

    Parameters:
    path (Path): The directory where the text files are located.
    index_to_drop (int): The index to drop from the dataframes.
    export (bool): Whether to export the results to CSV files. Default is True.

    Returns:
    concentration (DataFrame), response (DataFrame): The processed data.
    """
    path = Path(path)
    paths_to_data = []
    sample_names = []

    for file in path.glob("*/a-all.txt"):
        paths_to_data.append(file)
        sample_names.append(file.parent.stem)

    COLSPECS = [(0, 6), (7, 19), (20, 40), (41, 46), (47, 55), (56, 64)]

    data_holder = []
    for path_to_data in paths_to_data:
        df = pd.read_fwf(
            path_to_data,
            colspecs=COLSPECS,
            skiprows=19,
            skipfooter=5,
            names=["number", "compound", "Rt", "Qion", "Response", "Concentration"],
        )
        data_holder.append(df)

    frame = pd.concat(data_holder, axis=1, ignore_index=True)

    # Drop the 7th row:
    if index_to_drop:
        frame.drop(index=index_to_drop, inplace=True)

    # Separate concentration and response values to two dfs, plus compound names:
    concentration = frame.iloc[:, 5::6]
    response = frame.iloc[:, 4::6]
    compound = frame.iloc[:, 1]

    dataframes = [concentration, response]

    for df in dataframes:
        df.columns = sample_names
        df.index = compound
        df.index.name = "Response_ID"
        df[:] = df.apply(pd.to_numeric, errors="coerce")

    if export:
        concentration.to_csv(f"{path}/concentration.csv")
        response.to_csv(f"{path}/response.csv")
        print(f"The files were exported to {path}.")

    return concentration, response


@click.command()
@click.option(
    "--path",
    type=click.Path(exists=True),
    help='Specify the directory containing the GC files. Each folder should contain a file named "a-all.txt". Example: "C:/Users/my_username/my_GC_project"',
)
@click.option(
    "--index_to_drop",
    type=int,
    help="If the dataframes contain non-numeric data, use this option to locate and remove the corresponding row.",
)
@click.option(
    "--export",
    type=bool,
    default=True,
    help="Choose whether to export the resulting dataframes as CSV files. The default setting is True.",
)
def main(path, index_to_drop, export):
    """This program scans a specified directory for GC result files named 'a-all.txt',
    processes the data, and optionally exports the concentration and response results to CSV files.
    """
    concentration, response = process_and_export_GC_data(
        path=path, index_to_drop=index_to_drop, export=export
    )
    click.echo("Processing complete!")
    click.echo("Here is your dataframe representing the concentration values:")
    click.echo(concentration)
    click.echo("Here is your dataframe representing the response/peak area values:")
    click.echo(response)


if __name__ == "__main__":
    main()
