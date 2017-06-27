// original source for these enhancements to duration:
// https://github.com/sfackler/rust-time2/blob/90efd5b5794d9921ab5ff67ab99850e384557a0f/src/duration.rs
use std::time::Duration;

const NANOS_PER_SEC: f64 = 1_000_000_000.0;

/// A collection of utility extensions to `std::time::Duration`.
pub trait DurationExt: Sized {
    fn as_subsec_millis(&self) -> f64;
}

impl DurationExt for Duration {
    fn as_subsec_millis(&self) -> f64 {
        self.subsec_nanos() as f64 / NANOS_PER_SEC
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::DurationExt;

    #[test]
    fn it_works() {
        let my_duration: Duration = Duration::new(5, 3);
        let as_subsec_millis = my_duration.as_subsec_millis();
        assert_eq!(0.000000003, as_subsec_millis);
    }
}


use ggez::GameResult;
use event::Assets;

/// Play the 'click' sound effect. This is a general utility method as it is
/// used in a couple of different game states.
pub fn play_click_sfx(assets: &Assets) -> GameResult<()> {
    Ok(assets.get_sfx("click")?.play()?)
}
