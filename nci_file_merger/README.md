A command-line utility designed to extract concentration and peak area data from files generated on the Agilent 5973 (NCI).

Usage: nci_file_merger.exe [OPTIONS] --path <path>

Options:
  -p, --path <path>
          The root directory of the project. For instance, C:\Users\myname\myproject.
  -f, --file <file>
          The common file name that contains the peak areas and concentration data. Default is 'a-all.txt'. [default: a-all.txt]
      --save <save>
          Determines whether the files should be saved. Default is true. [default: true]
      --name_starts <name_starts>
          The starting position of the name column. Default is 8. [default: 8]
      --name_width <name_width>
          The width of the name column. Default is 20. [default: 20]
      --response_starts <response_starts>
          The starting position of the response column. Default is 49. [default: 49]
      --response_width <response_width>
          The width of the response column. Default is 6. [default: 6]
      --conc_starts <conc_starts>
          The starting position of the concentration column. Default is 56. [default: 56]
      --conc_width <conc_width>
          The width of the concentration column. Default is 9. [default: 9]
      --rows_to_skip_beginning <rows_to_skip_beginning>
          The number of rows to skip at the beginning of the file. Default is 19. [default: 19]
      --rows_to_take <rows_to_take>
          The number of rows to process from the top of the file. Default is 25. [default: 25]
  -h, --help
          Print help
  -V, --version
          Print version

# Example usage
1. Navigate to the folder, where the nci_file_merger.exe is located:
`powershell
cd "C:\Users\MyUserName\Folder_nci_file_merger"
`
2. Run the nci_file_merger CLI with the desired settings:
`powershell
.\nci_file_merger --path "C:\Users\MyUserName\MyProject" --rows_to_skip_beginning 5 --rows_to_take 20
`
