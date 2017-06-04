// original source for these enhancements to duration:
// https://github.com/sfackler/rust-time2/blob/90efd5b5794d9921ab5ff67ab99850e384557a0f/src/duration.rs
use std::time::Duration;

const NANOS_PER_SEC: u64 = 1_000_000_000;
const MILLIS_PER_SEC: u64 = 1_000;
const NANOS_PER_MILLI: u64 = NANOS_PER_SEC / MILLIS_PER_SEC;

pub trait DurationExt: Sized {
    fn as_millis(&self) -> Option<u64>;
}

impl DurationExt for Duration {
    fn as_millis(&self) -> Option<u64> {
        self.as_secs()
            .checked_mul(MILLIS_PER_SEC)
            .and_then(|m| m.checked_add(self.subsec_nanos() as u64 / NANOS_PER_MILLI))
    }
}
