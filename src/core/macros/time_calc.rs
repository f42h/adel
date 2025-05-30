// Specified by "DELETE_AFTER_N" setting:
// Calculate how long to wait before clearing 
// stored files and directories in ~/.adel
#[macro_export]
macro_rules! deletedelay {
    ($n:expr) => {
        $n * 24 * 60 * 60
    };
}

// Specified by "DELAY_HOUR", "DELAY_MIN", "DELAY_SEC" settings:
// Calculate how long to wait before starting a new scan in the 
// directories specified by the "ADEL_DIRS" setting
#[macro_export]
macro_rules! scandelay { 
    ($hour:expr, $min:expr, $sec:expr) => {
        ($hour * 3600) + ($min * 60) + $sec
    }
}