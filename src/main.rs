use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use async_std::task;
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

async fn wait_for_internet(timeout_length: Option<u64>, wait_time: Duration) {
    let start_time = SystemTime::now(); // remember start time for timeout
    loop {
        // exit if we're online
        match online(None).await {
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

        // Exit if we reach timeout
        if let Some(timeout_length) = timeout_length {
            let time_elapsed = start_time
                .elapsed()
                .expect("unexpected system time error...");
            if time_elapsed > Duration::from_secs(timeout_length) {
                eprintln!("Reached timeout of {} seconds!", timeout_length);
                exit(1);
            }
        }

        // sleep between checks
        if wait_time.as_secs() > 0 {
            sleep(wait_time);
        }
    }
}

fn main() {
    let opt = CLI::from_args(); // parse command line args
    let wait_time = Duration::from_secs(opt.wait); // duration to wait
    if opt.text.chars().count() > 0 {
        println!("{}", opt.text);
    }

    task::block_on(wait_for_internet(opt.timeout, wait_time))
}
