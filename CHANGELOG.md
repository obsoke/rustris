# 2.0.0

## Game-related changes

* Tetromino colours were updated. Ghost pieces for T and J are now easier to see on a dark background.

## Code-related changes

* Updated `ggez` to 0.4.X. This required many changes, including moving from a center-origin to top-left origin.
* Updated `rand` from 0.3.15 to 0.6
* Removed the `sdl2` crate as a dependency.
* Custom version of ggez's `events` module was removed. A single ggez-style scene, `SceneManager`, is now in charge of the substates using the new trait `State`.
* Now using Rust 2018
* Implemented fixes suggested by `clippy`.

# 1.0.1

* Fix: Bug where intro screen was too self-deprecating.

# 1.0.0

* Initial release.
