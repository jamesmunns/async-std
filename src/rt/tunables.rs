use once_cell::sync::Lazy;
use std::env::var;
use std::str::FromStr;

macro_rules! tunables {
    [$(($env_name:expr, $var_name:ident, $ty_name: ty, $default_val:expr),)+] => {
        pub struct Tunables {
            $(
                pub $var_name: $ty_name,
            )+
        }

        pub static TUNABLES: Lazy<Tunables> = Lazy::new(|| {
            Tunables {
                $(
                    $var_name: get_and_parse_or($env_name, $default_val),
                )+
            }
        });
    }
}

fn get_and_parse_or<T: FromStr>(key: &str, or: T) -> T {
    var(key)
        .ok()
        .and_then(|s| {
            str::parse::<T>(&s).ok()
        })
        .unwrap_or(or)
}

/// The global tunables
tunables!{
    ("ASYNC_STD_BLOCKING_DETECTION_INHIBIT",     blocking_detection_inhibit,     bool,  false),
    ("ASYNC_STD_BLOCKING_DETECTION_DELAY_US",    blocking_detection_us_delay,    u64,   10_000),
    ("ASYNC_STD_BLOCKING_DETECTION_INTERVAL_US", blocking_detection_us_interval, u64,   1_000),
    ("ASYNC_STD_MAX_PROCS",                      max_processes,                  usize, 512),
}
