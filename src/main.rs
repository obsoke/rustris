mod states;
mod util;

use crate::states::StateManager;
use ggez::graphics::Rect;
use ggez::{conf, event, graphics, ContextBuilder};
use std::{env, path};

fn main() {
    let mut cb = ContextBuilder::new("rustris", "obsoke")
        .window_setup(conf::WindowSetup::default().title("Rustris"))
        .window_mode(conf::WindowMode::default().dimensions(1280, 720));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        // We need this re-assignment alas, see
        // https://aturon.github.io/ownership/builders.html
        // under "Consuming builders"
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();

    // Our logical render target is 1280x720
    graphics::set_screen_coordinates(ctx, Rect::new_i32(0, 0, 1280, 720))
        .expect("Could not set logical screen coordinates before running initial state.");

    // Initialize & run the state manager
    let mut state = StateManager::new(ctx);
    if let Err(e) = event::run(ctx, &mut state) {
        println!("Error encountered in run: {}", e);
    }
}
