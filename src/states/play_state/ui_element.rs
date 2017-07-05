use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, Color, Rect, Point};
use event::Assets;
use states::play_state::tetromino::{Piece, PieceType};

const WIDTH: f32 = 170.0;
const HEIGHT: f32 = 50.0;
// text view's have their own height
const UITEXTVIEW_HEIGHT: f32 = 35.0;
const CIRCLE_RADIUS: f32 = 25.0;

// Ideally, there would be a general `View` trait or type or whatnot that I
// could implement these two views in a DRY way but UI design/creation wasn't
// one of my goals for this project so I just wanted to whip up something quick
// and esy.

/// Represents a UI element that displays a String.
pub struct UITextView {
    top_left: Point,
    centre: Point,
    title_text: graphics::Text,
    value: String,
    value_text: graphics::Text,
}

impl UITextView {
    /// Creates a new `UITextView` with the element title of `title` and a
    /// default value of `value.`
    pub fn new(
        ctx: &mut Context,
        assets: &Assets,
        top_left: Point,
        title: &str,
        value: &str,
    ) -> Self {
        let title_txt = graphics::Text::new(ctx, title, assets.get_font("normal").unwrap())
            .unwrap();
        let value_txt = graphics::Text::new(ctx, value, assets.get_font("normal").unwrap())
            .unwrap();
        Self {
            // ggez draws coords at center; we want to use top left as reference
            top_left: Point::new(
                top_left.x + (WIDTH / 2.0),
                top_left.y + (UITEXTVIEW_HEIGHT / 2.0),
            ),
            centre: top_left,
            title_text: title_txt,
            value: value.to_string(),
            value_text: value_txt,
        }
    }

    /// Currently updates the current value displayed. Only updates the value if
    /// it is different from the current value.
    pub fn update(&mut self, ctx: &mut Context, assets: &Assets, new_value: &str) {
        if new_value != self.value {
            self.value.clear(); // is this necessary?
            self.value = new_value.to_string();
            self.value_text =
                graphics::Text::new(ctx, new_value, assets.get_font("normal").unwrap()).unwrap();
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // title portion
        let title_rect = Rect::new(self.top_left.x, self.top_left.y, WIDTH, UITEXTVIEW_HEIGHT);
        let title_point = Point::new(self.top_left.x as f32, self.top_left.y as f32);
        graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, DrawMode::Fill, title_rect)?;
        graphics::circle(
            ctx,
            DrawMode::Fill,
            Point::new(self.centre.x as f32 + WIDTH as f32, self.top_left.y as f32),
            CIRCLE_RADIUS,
            32,
        )?;
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_point, 0.0)?;

        // value portion
        let value_rect = Rect::new(
            self.top_left.x - 20.0,
            self.top_left.y + UITEXTVIEW_HEIGHT,
            WIDTH - 40.0,
            UITEXTVIEW_HEIGHT,
        );
        let value_point = Point::new(
            self.top_left.x as f32,
            self.top_left.y as f32 + UITEXTVIEW_HEIGHT as f32,
        );
        graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, DrawMode::Fill, value_rect)?;
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.value_text, value_point, 0.0)?;
        Ok(())
    }
}

/// A UI element that renders `Pieces`. Used for elements like 'Next' or 'Hold'.
pub struct UIBlockView {
    top_left: Point,
    centre: Point,
    title_text: graphics::Text,
    shape: Option<PieceType>,
}

impl UIBlockView {
    pub fn new(
        ctx: &mut Context,
        assets: &Assets,
        top_left: Point,
        title: &str,
        shape: Option<PieceType>,
    ) -> Self {
        let title_txt = graphics::Text::new(ctx, title, assets.get_font("normal").unwrap())
            .unwrap();
        Self {
            // ggez draws coords at center; we want to use top left as reference
            top_left: Point::new(top_left.x + (WIDTH / 2.0), top_left.y + (HEIGHT / 2.0)),
            centre: top_left,
            title_text: title_txt,
            shape: shape,
        }
    }

    pub fn update(&mut self, _: &mut Context, _: &Assets, new_value: Option<PieceType>) {
        if self.shape != new_value {
            self.shape = new_value;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        // title portion
        let title_rect = Rect::new(self.top_left.x, self.top_left.y, WIDTH, HEIGHT);
        let title_point = Point::new(self.top_left.x as f32, self.top_left.y as f32);
        graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, DrawMode::Fill, title_rect)?;
        graphics::circle(
            ctx,
            DrawMode::Fill,
            Point::new(self.centre.x as f32 + WIDTH as f32, self.top_left.y as f32),
            CIRCLE_RADIUS,
            32,
        )?;
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_point, 0.0)?;

        // value portion
        let value_rect = Rect::new(
            self.top_left.x - 20.0,
            self.top_left.y + HEIGHT + (HEIGHT / 2.0),
            WIDTH - 40.0,
            HEIGHT * 2.0,
        );
        graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, DrawMode::Line, value_rect)?;
        if let Some(shape) = self.shape {
            let piece = Piece::new(shape);
            // the magic numbers below were derived by fiddling with values
            // until it looked right
            let next_piece_pos = if piece.get_type() == PieceType::I {
                Point::new(self.top_left.x - 95.0, self.top_left.y + 45.0)
            } else {
                Point::new(self.top_left.x - 110.0, self.top_left.y + 45.0)
            };
            piece.draw_at_point(
                ctx,
                assets.get_image("block")?,
                next_piece_pos,
                0.0,
            )?;
        }
        Ok(())
    }
}
