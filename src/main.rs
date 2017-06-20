extern crate ggez;
extern crate sdl2;

mod states;
mod event;
mod util;

use ggez::{Context, conf};
use event::run;

fn main() {
    let mut conf = conf::Conf::new();
    conf.window_title = String::from("Rustris");
    conf.window_width = 1280;
    conf.window_height = 720;
    conf.vsync = true;

    let ctx = &mut Context::load_from_conf("rustris", "obsoke", conf)
        .expect("Could not load configuartion");

    if let Err(e) = run(ctx) {
        println!("Error encountered: {}", e);
    }
}
