use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use online::online;
use structopt::StructOpt;

#[derive(StructOpt)]
/// Command line utility that waits till you have an internet connection.
struct CLI {
    #[structopt(short = "t", long)]
    /// Exits if a successful connection
    /// is not made within <timeout> seconds
    timeout: Option<u64>,
    #[structopt(short = "w", long = "--wait-time", default_value = "1")]
    /// Time to wait between failed requests
    wait: u64,
    #[structopt(long, default_value = "")]
    /// Text to display while waiting
    text: String,
}

fn main() {
    let opt = CLI::from_args(); // parse command line args
    let start_time = SystemTime::now(); // remember start time for timeout
    let wait_time = Duration::from_secs(opt.wait); // duration to wait
    if opt.text.chars().count() > 0 {
        println!("{}", opt.text);
    }

    loop {
        // Exit if we reach timeout
        if let Some(timeout_length) = opt.timeout {
            let time_elapsed = start_time.elapsed().unwrap(); // panics on std::time::SystemTimeError
            if time_elapsed > Duration::from_secs(timeout_length) {
                panic!("Reached timeout of {} seconds!", timeout_length);
            }
        }

        // exit if we're online
        match online(None) {
            // default 3 second timeout
            Ok(res) => {
                if res {
                    exit(0);
                }
            }
            Err(e) => {
                eprintln!("Warning: {}", e);
            } // ping failed, try again
        }

        // sleep between checks
        if opt.wait > 0 {
            sleep(wait_time);
        }
    }
}
