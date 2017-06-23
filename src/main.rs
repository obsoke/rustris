extern crate ggez;
extern crate sdl2;
extern crate rand;

mod states;
mod event;
mod util;

use ggez::{Context, conf, graphics};
use event::run;

fn main() {
    let mut conf = conf::Conf::new();
    conf.window_title = String::from("Rustris");
    conf.window_width = 1280;
    conf.window_height = 720;
    conf.vsync = true;

    let ctx = &mut Context::load_from_conf("rustris", "obsoke", conf).expect(
        "Could not load configuartion",
    );

    // Our logical render target is 1280x720
    graphics::set_screen_coordinates(ctx, 0.0, 1280.0, 0.0, 720.0).expect(
        "Could not set logical screen coordinates before running initial state.",
    );

    if let Err(e) = run(ctx) {
        println!("Error encountered in run: {}", e);
    }
}
