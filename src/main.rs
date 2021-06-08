use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use async_std::task;
use online::online;
use structopt::StructOpt;

#[derive(StructOpt)]
/// Command line utility that waits till you have an internet connection.
struct Cli {
    #[structopt(short = "t", long)]
    /// Exits if a successful connection
    /// is not made within <timeout> seconds
    timeout: Option<u64>,
    #[structopt(short = "q", long)]
    /// Don't print any warning/log messages
    quiet: bool,
    #[structopt(short = "w", long = "--wait-time", default_value = "1")]
    /// Time to wait between failed requests
    wait: u64,
    #[structopt(long, default_value = "")]
    /// Text to display while waiting
    text: String,
}

fn log(message: String, quiet: bool) {
    if !quiet {
        eprintln!("{}", message)
    }
}

async fn wait_for_internet(timeout_length: Option<u64>, wait_time: Duration, quiet: bool) {
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
                log(format!("Warning: {}", e), quiet);
            } // ping failed, try again
        }

        // Exit if we reach timeout
        if let Some(timeout_length) = timeout_length {
            let time_elapsed = start_time
                .elapsed()
                .expect("unexpected system time error...");
            if time_elapsed > Duration::from_secs(timeout_length) {
                log(
                    format!("Reached timeout of {} seconds!", timeout_length),
                    quiet,
                );
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
    let opt = Cli::from_args(); // parse command line args
    let wait_time = Duration::from_secs(opt.wait); // duration to wait
    if opt.text.chars().count() > 0 {
        log(opt.text, opt.quiet)
    }

    task::block_on(wait_for_internet(opt.timeout, wait_time, opt.quiet))
}
