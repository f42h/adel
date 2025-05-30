// (self.configs.delay_hour * 3600) + (self.configs.delay_min * 60) + self.configs.delay_sec

#[macro_export]
macro_rules! deletedelay {
    ($n:expr) => {
        $n * 24 * 60 * 60
    };
}

#[macro_export]
macro_rules! scandelay { // Delete after 
    ($hour:expr, $min:expr, $sec:expr) => {
        ($hour * 3600) + ($min * 60) + $sec
    }
}