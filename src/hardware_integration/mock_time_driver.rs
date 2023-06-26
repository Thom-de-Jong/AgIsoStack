
use core::time::Duration;

use super::TimeDriverTrait;

lazy_static::lazy_static! {
	static ref STARTUP_TIME: Duration = Duration::from_millis(0);
}

pub struct MockTimeDriver {}

impl TimeDriverTrait for MockTimeDriver {
    fn time_elapsed() -> Duration {
        STARTUP_TIME.saturating_add(Duration::from_millis(20))
    }
}
