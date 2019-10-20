use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::process::exit;

use structopt::StructOpt;
use online::*;

#[derive(StructOpt, Debug)]
/// Command line utility that waits till you have an internet connection.
struct CLI {

    #[structopt(short = "t", long)]
    /// Exits if a successful connection is not made within timeout seconds.
    timeout: Option<u64>,

    #[structopt(short = "u", long, default_value = "https://www.google.com/")]
    /// URL to check internet connection against
    url: String,

    #[structopt(short = "w", long = "--wait-time", default_value = "0")]
    /// Time to wait between failed requests
    wait: u64,
}


fn main() {

    let opt = CLI::from_args();
    let start_time = SystemTime::now();
    let wait_time = Duration::from_secs(opt.wait);

    loop {

        // Exit if we reach timeout
        match opt.timeout {
            Some(timeout_length) => {
                let time_elapsed = start_time.elapsed().unwrap(); // panics on std::time::SystemTimeError
                if time_elapsed > Duration::from_secs(timeout_length) {
                    panic!("Reached timeout of {} seconds!", timeout_length);
                }
            },
            None => (), // if no timeout was passed, wait infinitely
        }

        // exit if we're online
        match online(None) { // default 3 second timeout
            Ok(res) => {
                if res { // if response was successful
                    exit(0);
                }
            }
            Err(e) => {
                eprintln!("Warning: {}", e);
            }, // ping failed, try again
        }

        // sleep between checks
        if opt.wait > 0 {
            sleep(wait_time);
        }
    }
}
