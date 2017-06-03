use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Color, DrawMode, Rect};
use super::{BLOCK_SIZE, Position};
use super::shapes::*;

/// A `PieceShape` is a 4x4 array that represents the shape of a piece. A 0 is
/// an empty space while a 1 is solid.
pub type PieceShape = [[u32; 4]; 4];

/// All the possible piece types that can be taken out of `PieceBag`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    L,
    T,
    I,
    Z,
    J,
    S,
    O,
}

pub struct Piece {
    shape: PieceShape,
    shape_type: PieceType,
    pub top_left: Position,
    pub potential_top_left: Position,
    current_rotation_index: u32,
}

impl Piece {
    pub fn new(shape_type: PieceType) -> Self {
        let shape = piece_type_to_shape(shape_type, 0);

        Piece {
            shape: shape,
            shape_type: shape_type,
            top_left: Position { x: 4, y: 0 },
            potential_top_left: Position { x: 4, y: 0 },
            current_rotation_index: 0,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // get starting position to draw window
        // TODO: doing all of this work every frame seems bad
        let width = graphics::get_screen_coordinates(&ctx).w;
        let middle = width / 2.0;
        let starting_pos = middle - ((BLOCK_SIZE as f32 * 10.0) / 2.0);

        for r in 0 .. self.shape.len() {
            for c in 0 .. self.shape[r].len() {
                if self.shape[r][c] != 0 {
                    if r + (self.top_left.y as usize) < 2 { // don't draw in vanish zone
                        continue;
                    }
                    match self.shape[r][c] {
                        1 => graphics::set_color(ctx, Color::from((255, 0, 0)))?,
                        2 => graphics::set_color(ctx, Color::from((0, 255, 0)))?,
                        3 => graphics::set_color(ctx, Color::from((0, 0, 255)))?,
                        4 => graphics::set_color(ctx, Color::from((255, 0, 255)))?,
                        5 => graphics::set_color(ctx, Color::from((255, 255, 0)))?,
                        6 => graphics::set_color(ctx, Color::from((0, 255, 255)))?,
                        7 => graphics::set_color(ctx, Color::from((255, 255, 255)))?,
                        _ => unreachable!(),
                    }

                    graphics::rectangle(ctx, DrawMode::Fill, Rect {
                        x: (starting_pos + (c as f32 + self.top_left.x as f32)) * BLOCK_SIZE as f32,
                        y: (r as f32 + self.top_left.y as f32) * BLOCK_SIZE as f32,
                        w: BLOCK_SIZE as f32,
                        h: BLOCK_SIZE as f32,
                    });
                }
            }
        }

        Ok(())
    }

    pub fn draw_shadow(&self, ctx: &mut Context, shadow_position: &Position) -> GameResult<()> {
        // get starting position to draw window
        // TODO: doing all of this work every frame seems bad
        let width = graphics::get_screen_coordinates(&ctx).w;
        let middle = width / 2.0;
        let starting_pos = middle - ((BLOCK_SIZE as f32 * 10.0) / 2.0);

        for (r, _) in self.shape.iter().enumerate() {
            for (c, _) in self.shape[r].iter().enumerate() {
                if self.shape[r][c] != 0 {
                    match self.shape[r][c] {
                        1 => graphics::set_color(ctx, Color::from((255, 0, 0)))?,
                        2 => graphics::set_color(ctx, Color::from((0, 255, 0)))?,
                        3 => graphics::set_color(ctx, Color::from((0, 0, 255)))?,
                        4 => graphics::set_color(ctx, Color::from((255, 0, 255)))?,
                        5 => graphics::set_color(ctx, Color::from((255, 255, 0)))?,
                        6 => graphics::set_color(ctx, Color::from((0, 255, 255)))?,
                        7 => graphics::set_color(ctx, Color::from((255, 255, 255)))?,
                        _ => unreachable!(),
                    }

                    graphics::rectangle(ctx, DrawMode::Fill, Rect {
                        x: (starting_pos + (c as f32 + shadow_position.x as f32)) * BLOCK_SIZE as f32,
                        y: (r as f32 + shadow_position.y as f32) * BLOCK_SIZE as f32,
                        w: BLOCK_SIZE as f32,
                        h: BLOCK_SIZE as f32,
                    });
                }
            }
        }

        Ok(())
    }

    // this only returns the next shape, it doesn't change the current shape
    pub fn get_next_shape(&self) -> PieceShape {
        let next_index = next_rotation_index(self.current_rotation_index, 1);

        piece_type_to_shape(self.shape_type, next_index as usize)
    }

    pub fn get_shape(&self) -> PieceShape {
        self.shape
    }

    pub fn change_shape(&mut self) {
        let next_index = next_rotation_index(self.current_rotation_index, 1);

        self.shape = piece_type_to_shape(self.shape_type, next_index as usize);
        self.current_rotation_index = next_index;
    }
}

/*
 * Utility methods
*/

pub fn piece_type_to_shape(shape: PieceType, index: usize) -> PieceShape {
    match shape {
        PieceType::O => O_SHAPES[index],
        PieceType::J => J_SHAPES[index],
        PieceType::I => I_SHAPES[index],
        PieceType::S => S_SHAPES[index],
        PieceType::Z => Z_SHAPES[index],
        PieceType::L => L_SHAPES[index],
        PieceType::T => T_SHAPES[index],
    }
}

pub fn u8_to_piece_type(num: u8) -> Option<PieceType> {
    match num {
        0 => Some(PieceType::L),
        1 => Some(PieceType::T),
        2 => Some(PieceType::I),
        3 => Some(PieceType::Z),
        4 => Some(PieceType::J),
        5 => Some(PieceType::S),
        6 => Some(PieceType::O),
        _ => unreachable!(),
    }
}

pub fn next_rotation_index(current_index: u32, direction: i32) -> u32 {
    if direction == -1 {
        match current_index {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            _ => unreachable!(),
        }
    }
    else {
        match current_index {
            0 => 3,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!(),
        }
    }
}
