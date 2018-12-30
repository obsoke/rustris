use crate::states::Assets;
use ggez::graphics::{Color, Point2};
use ggez::{graphics, Context, GameResult};

/// Different representations of possible commands that could be received from
/// the user in the menu state.
pub enum OptionInputCommand {
    Up,
    Down,
    Select,
}

/// A menu option.
pub struct Option {
    text: graphics::Text,
    position: Point2,
    is_selected: bool,
}

impl Option {
    /// Creates a new `Option`.
    pub fn new(ctx: &mut Context, assets: &Assets, name: &'static str, top_left: Point2) -> Self {
        let text = graphics::Text::new(ctx, name, assets.get_font("normal").unwrap()).unwrap();
        let real_pos = Point2::new(top_left.x - (text.width() / 2) as f32, top_left.y);
        Self {
            text,
            position: real_pos,
            is_selected: false,
        }
    }

    /// Currently, this updates whether the `Option` is the currently selected
    /// `Option` or not.
    pub fn update(&mut self, is_selected: bool) -> GameResult<()> {
        if self.is_selected != is_selected {
            self.is_selected = is_selected;
        }
        Ok(())
    }

    /// Draws the `Option`. The colour of the text rendered depends on
    /// `is_selected`.
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.is_selected {
            graphics::set_color(ctx, Color::new(1.0, 1.0, 0.0, 1.0))?;
        } else {
            graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        }

        graphics::draw(ctx, &self.text, self.position, 0.0)?;

        Ok(())
    }
}
