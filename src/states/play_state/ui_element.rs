use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, Color, Rect, Point};
use event::Assets;
use states::play_state::Position;

const WIDTH: i32 = 200;
const HEIGHT: i32 = 50;
const CIRCLE_RADIUS: f32 = 35.0;

pub struct UIElement {
    top_left: Position,
    centre: Position,
    title_text: graphics::Text,
    value: String,
    value_text: graphics::Text,
}

impl UIElement {
    pub fn new(ctx: &mut Context, assets: &Assets, top_left: Position, title: &str, value: &str) -> Self {
        let title_txt = graphics::Text::new(ctx, title, assets.get_font("normal").unwrap()).unwrap();
        let value_txt = graphics::Text::new(ctx, value, assets.get_font("normal").unwrap()).unwrap();
        Self {
            // ggez draws coords at center; we want to use top left as reference
            top_left: Position::new(top_left.x + (WIDTH / 2), top_left.y + (HEIGHT / 2)),
            centre: top_left,
            title_text: title_txt,
            value: value.to_string(),
            value_text: value_txt,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, assets: &Assets, new_value: &str) {
        if new_value != self.value {
            self.value.clear(); // is this necessary?
            self.value = new_value.to_string();
            self.value_text = graphics::Text::new(ctx, new_value, assets.get_font("normal").unwrap()).unwrap();
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // title portion
        let title_rect = Rect::new_i32(self.top_left.x, self.top_left.y, WIDTH, HEIGHT);
        let title_point = Point::new(self.top_left.x as f32, self.top_left.y as f32);
        graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, DrawMode::Fill, title_rect)?;
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point::new(self.centre.x as f32 + WIDTH as f32, self.top_left.y as f32),
                         CIRCLE_RADIUS,
                         32)?;

        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_point, 0.0)?;

        // value portion
        let value_rect = Rect::new_i32(self.top_left.x - 20, self.top_left.y + HEIGHT, WIDTH - 40, HEIGHT);
        let value_point = Point::new(self.top_left.x as f32, self.top_left.y as f32 + HEIGHT as f32);
        graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, DrawMode::Fill, value_rect)?;
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.value_text, value_point, 0.0)?;
        Ok(())
    }
}
