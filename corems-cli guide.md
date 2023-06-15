## corems-cli: A Command-Line Tool for Molecular Formula Assignment of HR-LC-MS Data

### Introduction

This command-line tool leverages a containerized version of CoreMS for formula assignment of high resolution accurate mass (FT-ICR, Orbitrap) data. The tool reads text-based input files or user-provided Python scripts that define assignment parameters. It can be run from any directory with .RAW files, an input file (or Python script), and a calibration file (all are required for assignments). 

### Installation

1. Download the binary package compatible with your operating system here: https://github.com/deweycw/corems-cli/releases/tag/beta

2. Extract the contents of the downloaded folder (this may happen automatically). 

> - You may see some warnings about downloading software from the internet. You can bypass these warnings for this package (it's safe). 

3. Double-click the **INSTALL** executable. 


### Flags

`-i/--input_file:       <Optional> name of input file with assignment parameters; default value is "corems.in"`

`-s/--script:           <Optional> name of user provided Python script with assignment parameters; cannot be used with -i flag`

`-c/--container:        <Optional> namne of container to use for assignment; default value is "corems-cli"`



### Workflow

1. Open a Terminal (Mac) or a Powershell (Windows).

2. Navigate to the folder with your .RAW files, an input file (or Python script), and a calibration file. 

3. Run `corems-cli assign` from the Terminal or Powershell window to perform assignments. Add optional flags if desired. 







