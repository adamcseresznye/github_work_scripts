# NCI File Merger
A command-line utility written in Rust 🦀, designed to extract concentration and peak area data from files generated on the Agilent 5973 (NCI).

# 📖 Instructions for Use

1. Open Powershell and navigate to the directory containing the downloaded `nci_file_merger.exe` executable. For example:  
   `cd "C:\Users\MyUserName\Folder_nci_file_merger"`  

2. Run the NCI File Merger command-line interface (CLI) with the desired parameters:  
   `.\nci_file_merger --path "C:\Users\MyUserName\Folder_gc_files" --rows_to_skip_beginning 5 --rows_to_take 20`  
   In this example, `--path` specifies the root directory of the folder containing the data files.

3. Initially, you can adjust the desired parameters by setting the `--save` option to `false`. This will print the result to the terminal, allowing for quicker iteration to find the most optimal parameters (such as start and width of columns). 

4. Once you're satisfied with the result, you can save the results to CSV files by setting `--save` to `true`.

## 🤔 Options:
**Please Note**: A comprehensive list of available options can be obtained by executing `.\nci_file_merger --help`.

- `-p, --path <path>`  
  The root directory of the project. For instance, `C:\Users\myname\myproject`.

- `-f, --file <file>`  
  The common file name that contains the peak areas and concentration data. Default is 'a-all.txt'. [default: a-all.txt]

- `--save <save>`  
  Set to true to save files to CSV. Set to false to display files on the command line. Default is true. [default: true]

- `--name_starts <name_starts>`  
  The starting position of the name column. Default is 8. [default: 8]

- `--name_width <name_width>`  
  The width of the name column. Default is 20. [default: 20]

- `--response_starts <response_starts>`  
  The starting position of the response column. Default is 49. [default: 49]

- `--response_width <response_width>`  
  The width of the response column. Default is 6. [default: 6]

- `--conc_starts <conc_starts>`  
  The starting position of the concentration column. Default is 56. [default: 56]

- `--conc_width <conc_width>`  
  The width of the concentration column. Default is 9. [default: 9]

- `--rows_to_skip_beginning <rows_to_skip_beginning>`  
  The number of rows to skip at the beginning of the file. Default is 19. [default: 19]

- `--rows_to_take <rows_to_take>`  
  The number of rows to process from the top of the file. Default is 25. [default: 25]

- `-h, --help`  
  Print help

- `-V, --version`  
  Print version


