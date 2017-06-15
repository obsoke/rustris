extern crate ggez;
extern crate sdl2;

mod states;

use ggez::{Context, GameResult, conf, timer};
use ggez::event::EventHandler;
use sdl2::{keyboard, event as SdlEvent};
use sdl2::event::Event::*;

use states::play_state::PlayState;
use states::StateManager;

fn main() {
    let mut conf = conf::Conf::new();
    conf.window_title = String::from("Rustris");
    conf.window_width = 1280;
    conf.window_height = 720;
    conf.vsync = true;

    let ctx = &mut Context::load_from_conf("rustris", "obsoke", conf).expect("Could not load configuartion");

    let state = PlayState::new(ctx).expect("Could not initialize state");

    if let Err(e) = run(ctx, state) {
        println!("Error encountered: {}", e);
    }
}


/// Runs the game's main loop, calling event callbacks on the given state
/// object as events occur.
///
/// It does not try to do any type of framerate limiting.  See the
/// documentation for the `timer` module for more info.
pub fn run<S>(ctx: &mut Context, state: S) -> GameResult<()>
    where S: EventHandler + 'static
{
    {
        let mut state_manager = StateManager::new(state);
        let mut event_pump = ctx.sdl_context.event_pump()?;

        let mut continuing = true;
        while continuing {
            ctx.timer_context.tick();

            for event in event_pump.poll_iter() {
                match event {
                    Quit { .. } => {
                        continuing = state_manager.quit_event();
                        // println!("Quit event: {:?}", t);
                    }
                    KeyDown {
                        keycode,
                        keymod,
                        repeat,
                        ..
                    } => {
                        if let Some(key) = keycode {
                            if key == keyboard::Keycode::Escape {
                                ctx.quit()?;
                            } else {
                                state_manager.key_down_event(key, keymod, repeat)
                            }
                        }
                    }
                    KeyUp {
                        keycode,
                        keymod,
                        repeat,
                        ..
                    } => {
                        if let Some(key) = keycode {
                            state_manager.key_up_event(key, keymod, repeat)
                        }
                    }
                    MouseButtonDown { mouse_btn, x, y, .. } => {
                        state_manager.mouse_button_down_event(mouse_btn, x, y)
                    }
                    MouseButtonUp { mouse_btn, x, y, .. } => {
                        state_manager.mouse_button_up_event(mouse_btn, x, y)
                    }
                    MouseMotion {
                        mousestate,
                        x,
                        y,
                        xrel,
                        yrel,
                        ..
                    } => state_manager.mouse_motion_event(mousestate, x, y, xrel, yrel),
                    MouseWheel { x, y, .. } => state_manager.mouse_wheel_event(x, y),
                    ControllerButtonDown { button, which, .. } => {
                        state_manager.controller_button_down_event(button, which)
                    }
                    ControllerButtonUp { button, which, .. } =>
                        state_manager.controller_button_up_event(button, which),
                    ControllerAxisMotion { axis, value, which, .. } => {
                        state_manager.controller_axis_event(axis, value, which)
                    }
                    Window { win_event: SdlEvent::WindowEvent::FocusGained, .. } => {
                        state_manager.focus_event(true)
                    }
                    Window { win_event: SdlEvent::WindowEvent::FocusLost, .. } => {
                        state_manager.focus_event(false)
                    }
                    _ => {}
                }
            }

            let dt = timer::get_delta(ctx);
            state_manager.update(ctx, dt)?;
            state_manager.draw(ctx)?;
        }
    }

    Ok(())
}
