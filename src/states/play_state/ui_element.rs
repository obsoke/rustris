use crate::states::play_state::tetromino::{Piece, PieceType};
use crate::states::Assets;
use ggez::graphics::{Color, DrawMode, Point2, Rect};
use ggez::{graphics, Context, GameResult};

const WIDTH: f32 = 150.0;
const UIBLOCK_HEIGHT: f32 = 130.0;
const UITEXT_HEIGHT: f32 = 40.0;
const UI_BG_COLOUR: Color = Color {
    r: 0.3,
    b: 0.3,
    g: 0.3,
    a: 0.1,
};

// Ideally, there would be a general `View` trait or type or whatnot that I
// could implement these two views in a DRY way but UI design/creation wasn't
// one of my goals for this project so I just wanted to whip up something quick
// and esy.

/// Represents a UI element that displays a String.
pub struct UITextView {
    top_left: Point2,
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
        top_left: Point2,
        title: &str,
        value: &str,
    ) -> Self {
        let title_txt = graphics::Text::new(ctx, title, assets.get_font("ui").unwrap()).unwrap();
        let value_txt = graphics::Text::new(ctx, value, assets.get_font("ui").unwrap()).unwrap();
        Self {
            top_left,
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
                graphics::Text::new(ctx, new_value, assets.get_font("ui").unwrap()).unwrap();
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let title_rect = Rect::new(
            self.top_left.x,
            self.top_left.y,
            WIDTH + 100.0,
            UITEXT_HEIGHT,
        );
        let text_v_offset = 5.0;
        let title_point = Point2::new(
            self.top_left.x as f32 + 8.0,
            self.top_left.y as f32 + text_v_offset,
        );
        graphics::set_color(ctx, UI_BG_COLOUR)?;
        graphics::rectangle(ctx, DrawMode::Fill, title_rect)?;
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_point, 0.0)?;

        let val_width = self.value_text.width() as f32;
        let value_point = Point2::new(
            self.top_left.x as f32 + (WIDTH + 100.0 - val_width),
            self.top_left.y as f32 + text_v_offset,
        );
        graphics::set_color(ctx, Color::new(1.0, 1.0, 0.0, 1.0))?;
        graphics::draw(ctx, &self.value_text, value_point, 0.0)?;
        Ok(())
    }
}

/// A UI element that renders `Pieces`. Used for elements like 'Next' or 'Hold'.
pub struct UIBlockView {
    top_left: Point2,
    title_text: graphics::Text,
    shape: Option<PieceType>,
}

impl UIBlockView {
    pub fn new(
        ctx: &mut Context,
        assets: &Assets,
        top_left: Point2,
        title: &str,
        shape: Option<PieceType>,
    ) -> Self {
        let title_txt = graphics::Text::new(ctx, title, assets.get_font("ui").unwrap()).unwrap();
        Self {
            top_left,
            title_text: title_txt,
            shape,
        }
    }

    pub fn update(&mut self, _: &mut Context, _: &Assets, new_value: Option<PieceType>) {
        if self.shape != new_value {
            self.shape = new_value;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        // title portion
        let title_rect = Rect::new(self.top_left.x, self.top_left.y, WIDTH, UIBLOCK_HEIGHT);
        let title_point = Point2::new(self.top_left.x as f32 + 8.0, self.top_left.y as f32 + 5.0);
        graphics::set_color(ctx, UI_BG_COLOUR)?;
        graphics::rectangle(ctx, DrawMode::Fill, title_rect)?;
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_point, 0.0)?;

        if let Some(shape) = self.shape {
            let piece = Piece::new(shape);
            // the magic numbers below were derived by fiddling with values
            // until it looked right. ideally, some kind of object graph would
            // be used to make relative positioning easier
            let next_piece_pos = if piece.get_type() == PieceType::I {
                Point2::new(self.top_left.x - 10.0, self.top_left.y + 30.0)
            } else if piece.get_type() == PieceType::L {
                Point2::new(self.top_left.x - 10.0, self.top_left.y + 40.0)
            } else if piece.get_type() == PieceType::J {
                Point2::new(self.top_left.x, self.top_left.y + 35.0)
            } else {
                Point2::new(self.top_left.x - 10.0, self.top_left.y + 25.0)
            };
            piece.draw_at_point(ctx, assets.get_image("block")?, next_piece_pos, 0.0)?;
        }
        Ok(())
    }
}
