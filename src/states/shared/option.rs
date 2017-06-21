use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Point, Color};
use event::Assets;
/// A menu option.
pub struct Option {
    text: graphics::Text,
    centre: Point,
    is_selected: bool,
}

impl Option {
    pub fn new(ctx: &mut Context, assets: &Assets, name: &'static str, top_left: Point) -> Self {
        let text = graphics::Text::new(ctx, name, assets.get_font("normal").unwrap()).unwrap();
        Self {
            text: text,
            centre: top_left,
            is_selected: false,
        }
    }

    pub fn update(&mut self, is_selected: bool) -> GameResult<()> {
        if self.is_selected != is_selected {
            self.is_selected = is_selected;
        }
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.is_selected {
            graphics::set_color(ctx, Color::new(1.0, 1.0, 0.0, 1.0))?;
        } else {
            graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        }

        graphics::draw(ctx, &self.text, self.centre, 0.0)?;

        Ok(())
    }
}


