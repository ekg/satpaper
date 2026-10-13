#![feature(once_cell_try, isqrt)]

mod config;
mod slider;
mod wallpaper;

use std::time::Duration;
use std::thread::{self, sleep};
use std::sync::mpsc;

use anyhow::{Result, Context};
use clap::Parser;

use crate::config::*;

const OUTPUT_NAME: &str = "satpaper_latest.png";
const SLEEP_DURATION: Duration = Duration::from_secs(60);
/// Hard cap on a single check cycle (timestamp fetch + tile download +
/// compositing). If a network call wedges - e.g. an unresponsive resolver or a
/// stalled connection - the check is abandoned and retried next cycle instead
/// of freezing the update loop indefinitely.
const CHECK_TIMEOUT: Duration = Duration::from_secs(15 * 60);

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
    
    update_wallpaper()
        .context("An error occurred in the wallpaper updating loop")?;

    Ok(())
}

fn update_wallpaper() -> Result<()> {
    let config = Config::parse();
    
    let mut timestamp = None;
    
    loop {
        log::debug!("Checking timestamp...");

        // Run the check in a watched thread so that a wedged network call
        // cannot stall the update loop forever. An abandoned check thread
        // eventually finishes on its own and its result is simply discarded.
        let (tx, rx) = mpsc::channel();
        let check_config = config.clone();

        thread::spawn(move || {
            let _ = tx.send(run_check(&check_config, timestamp));
        });

        match rx.recv_timeout(CHECK_TIMEOUT) {
            Ok(Ok(CheckOutcome::Unchanged)) => {},
            Ok(Ok(CheckOutcome::Updated(new))) => timestamp = Some(new),
            Ok(Ok(CheckOutcome::Finished)) => return Ok(()),
            Ok(Err(err)) => return Err(err.context("An error occurred during a wallpaper check")),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                log::error!("Check did not finish within {CHECK_TIMEOUT:?}; abandoning it.");
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                log::error!("Check thread terminated unexpectedly; waiting until next go round.");
            }
        }

        log::debug!("Sleeping for {SLEEP_DURATION:?}...");

        sleep(SLEEP_DURATION);
    }

    #[allow(unreachable_code)]
    Ok(())
}

enum CheckOutcome {
    /// Nothing new, or the check failed benignly.
    Unchanged,
    /// A new wallpaper was composited and set; carries the new timestamp.
    Updated(u64),
    /// A wallpaper was composited in `--once` mode; the caller should exit.
    Finished,
}

fn run_check(config: &Config, old: Option<u64>) -> Result<CheckOutcome> {
    let new = slider::fetch_latest_timestamp(config)
        .unwrap_or_else(|err| {
            log::error!("Failed to fetch latest timestamp: {err}");
            log::error!("Check aborted; waiting until next go round.");
            old.unwrap_or(0)
        });

    if old
        .map_or(true, |old| old != new)
    {
        log::info!("Timestamp has changed!");
        log::debug!("Old timestamp: {old:?}, new timestamp: {new}");
        log::info!("Fetching updated source and compositing new wallpaper...");

        if slider::composite_latest_image(config)? {
            if config.once {
                return Ok(CheckOutcome::Finished);
            }

            wallpaper::set(
                config.target_path.join(OUTPUT_NAME),
                config.wallpaper_command.as_deref(),
            )?;

            log::info!("New wallpaper composited and set.");

            return Ok(CheckOutcome::Updated(new));
        }
    }

    Ok(CheckOutcome::Unchanged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_wallpaper() -> Result<()> {
        let config = Config {
            satellite: Satellite::GOESEast,
            resolution_x: 2556,
            resolution_y: 1440,
            disk_size: 95,
            target_path: ".".into(),
            wallpaper_command: None,
            once: false,
            background_image: None
        };

        slider::composite_latest_image(&config)?;

        std::fs::remove_file("./satpaper_latest.png")?;

        Ok(())
    }
}