This is a CLI application designed to read and process data files, specifically those named ‘a-all.txt’,
which were obtained from the old-GC. The processed data is then prepared for subsequent data analysis.

```
This program scans a specified directory for GC result files named
  'a-all.txt',  processes the data, and optionally exports the concentration
  and response results to CSV files.

Options:
  --path PATH              Specify the directory containing the GC files. Each
                           folder should contain a file named "a-all.txt".
                           Example: "C:/Users/my_username/my_GC_project"
  --index_to_drop INTEGER  If the dataframes contain non-numeric data, use
                           this option to locate and remove the corresponding
                           row.
  --export BOOLEAN         Choose whether to export the resulting dataframes
                           as CSV files. The default setting is True.
  --help                   Show this message and exit.
```

**Usage**
- Identify the directory containing the script.
- Set it as your working directory (for example cd C:\Users\my_username\GCDataProcessor).
- Execute the CLI application.

**Example usage as CLI**
```bash
 python GCDataProcessor.py --path="C:\Users\my_username\data_files" --index_to_drop=3 --export=True
 ```
