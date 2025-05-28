# ADEL 

### Description
```
The ADEL project helps to keep the file system clean by automatically removing
temporary/unwanted files from the specified directories in a temporary location. 
All files can be accessed in the ADEL home directory before they are permanently 
removed after N days.
```

### Usage
- Setting up
```conf
# ADEL - Auto Delete Temporary/Unwanted Files And Directories

# Configurations:

# NOTE: Leave UNAME in path to automatically get the 
# username from the system

# This is adels temporary directory, when something 
# is removed unwanted you can recover it from here
PATH_TEMP_DIR=/home/UNAME/.adel/

# Specify the directories where to expect 
# files/directories with the .adel extension
# (comma separated)
ADEL_DIRS=/home/UNAME/Downloads,/home/UNAME/Schreibtisch

# Configure the scan delay for directories
# listed in ADEL_DIRS
DELAY_HOUR=0
DELAY_MIN=0
DELAY_SEC=10

# Clear adels home after N days (delete moved .adel files)
DELETE_AFTER_N=7
```

- To start ADEL the helper script `adel.sh` can be used
```bash
# Execute ADEL with enabled logging

bash adel.sh
```

#### Every file and every directory ending with `.adel` will be moved to adels home directory `~/.adel`. From there all the files can be accessed until the DELETE_AFTER_N timer runs out. After that, every entry in ADELs home directory will be fully removed.