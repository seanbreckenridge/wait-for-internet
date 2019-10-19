use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::process::exit;

use structopt::StructOpt;
use reqwest;

#[derive(StructOpt, Debug)]
/// Command line utility that waits till you have an internet connection.
struct CLI {

    #[structopt(short = "t", long)]
    /// Exits if a successful connection is not made within timeout seconds.
    timeout: Option<u64>,

    #[structopt(short = "u", long, default_value = "https://www.google.com/")]
    /// URL to check internet connection against
    url: String,

    #[structopt(short = "w", long = "--wait-time", default_value = "1")]
    /// Time to wait between failed requests
    wait: u64,
}


fn main() {
    let opt = CLI::from_args();
    //println!("{:?}", opt);
    
    let start_time = SystemTime::now();

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

        // blocking get
        let resp = reqwest::get(&opt.url);
        match resp {
            Ok(r) => {
                println!("Succeeded with status code '{}'", r.status());
                exit(0);
            },
            Err(_) => (), // ignore errors
        }

        // Sleep
        sleep(Duration::from_secs(opt.wait));
        // println!("{:?}", start_time.elapsed().unwrap());
    }
}
