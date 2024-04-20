# nci_file_merger
A command-line utility designed to extract concentration and peak area data from files generated on the Agilent 5973 (NCI).

# Example usage

1. Navigate to the folder, where the nci_file_merger.exe is located:

`powershell
cd "C:\Users\MyUserName\Folder_nci_file_merger"
`
2. Run the nci_file_merger CLI with the desired settings:

`powershell
.\nci_file_merger --path "C:\Users\MyUserName\MyProject" --rows_to_skip_beginning 5 --rows_to_take 20
`
