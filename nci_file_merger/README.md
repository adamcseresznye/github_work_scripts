# NCI File Merger
A command-line utility designed to extract concentration and peak area data from files generated on the Agilent 5973 (NCI).

# Instructions for Use

1. Initiate Powershell and navigate to the directory containing the `nci_file_merger.exe` executable. For instance:  
   `cd "C:\Users\MyUserName\Folder_nci_file_merger"`  
3. Execute the NCI File Merger command-line interface (CLI) with the desired parameters:  
   `.\nci_file_merger --path "C:\Users\MyUserName\Folder_gc_files" --rows_to_skip_beginning 5 --rows_to_take 20`  
   In this example, `--path` specifies the root directory of the folder containing the data files.

**Please Note**: A comprehensive list of available options can be obtained by executing `.\nci_file_merger --help`.




