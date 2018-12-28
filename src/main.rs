extern crate ggez;
extern crate rand;

mod states;
mod util;

use crate::states::StateManager;
use ggez::{conf, event, graphics, Context};

fn main() {
    let mut conf = conf::Conf::new();
    conf.window_title = String::from("Rustris");
    conf.window_width = 1280;
    conf.window_height = 720;
    conf.vsync = true;

    let ctx = &mut Context::load_from_conf("rustris", "obsoke", conf)
        .expect("Could not load configuartion");

    // Our logical render target is 1280x720
    graphics::set_screen_coordinates(ctx, 0.0, 1280.0, 0.0, 720.0)
        .expect("Could not set logical screen coordinates before running initial state.");

    // Initialize & run the state manager
    let mut state = StateManager::new(ctx);
    if let Err(e) = event::run(ctx, &mut state) {
        println!("Error encountered in run: {}", e);
    }
}
