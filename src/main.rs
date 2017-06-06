extern crate ggez;

mod states;

use ggez::{Context, event, conf};
use states::play_state::PlayState;

fn main() {
    let mut conf = conf::Conf::new();
    conf.window_title = String::from("Rustris");
    conf.window_width = 1280;
    conf.window_height = 720;
    conf.vsync = true;

    let ctx = &mut Context::load_from_conf("rustris", "obsoke", conf).expect("Could not load configuartion");

    let state = &mut PlayState::new(ctx).expect("Could not initialize state");

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    }
}
