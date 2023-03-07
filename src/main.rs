use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use clap::Parser;
use online::check;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Command line utility that waits till you have an internet connection."
)]
struct Cli {
    /// Exits if a successful connection
    /// is not made within <timeout> seconds
    #[arg(short, long, default_value = None)]
    timeout: Option<u64>,

    /// Don't print any warning/log messages
    #[arg(short, long, default_value = "false")]
    quiet: bool,

    #[arg(short, long, default_value = "1")]
    /// Time to wait between failed requests
    wait_time: u64,

    #[arg(long, default_value = "")]
    /// Text to display while waiting
    text: String,
}

struct App {
    cli: Cli,
}

impl App {
    fn log(&self, message: String) {
        if !self.cli.quiet {
            eprintln!("{}", message)
        }
    }

    fn log_text(&self) {
        if !self.cli.text.is_empty() {
            self.log(self.cli.text.clone())
        }
    }

    fn wait_for_internet(&self) -> Result<i32> {
        self.log_text();
        let start_time = SystemTime::now();
        let wait = Duration::from_secs(self.cli.wait_time);

        loop {
            // exit if we're online
            match check(None) {
                // default 3 second timeout
                Ok(_) => return Ok(0),
                Err(e) => self.log(format!("Warning: {}", e)), // ping failed, try again
            }

            // Exit if we reach timeout
            if let Some(timeout_length) = self.cli.timeout {
                let time_elapsed = start_time
                    .elapsed()
                    .context("unexpected system time error")?;
                if time_elapsed > Duration::from_secs(timeout_length) {
                    self.log(format!("Reached timeout of {} seconds!", timeout_length));
                    return Ok(1);
                }
            }

            // sleep between checks
            if wait.as_secs() > 0 {
                sleep(wait);
            }
        }
    }
}

fn main() -> Result<()> {
    let app = App { cli: Cli::parse() };
    match app.wait_for_internet() {
        Ok(exit_code) => exit(exit_code),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
