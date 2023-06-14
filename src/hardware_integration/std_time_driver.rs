
use std::time::Instant;
use core::time::Duration;

use super::TimeDriverTrait;

lazy_static::lazy_static! {
	static ref STARTUP_TIME: Instant = Instant::now();
}

pub struct StdTimeDriver {}

impl TimeDriverTrait for StdTimeDriver {
    fn time_elapsed() -> Duration {
        	STARTUP_TIME.elapsed()
    }
}
