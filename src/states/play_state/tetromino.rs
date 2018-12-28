use super::shapes::*;
use super::well::Y_OFFSET;
use super::BLOCK_SIZE;
use ggez::graphics::{Color, DrawParam, Point};
use ggez::{graphics, Context, GameResult};

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

/// The tetromino piece that the player controls.
pub struct Piece {
    shape: PieceShape,
    shape_type: PieceType,
    current_rotation_index: u32,
    pub top_left: Point,
    pub potential_top_left: Point,
    shadow_position: Point,
}

impl Piece {
    /// Creates a new piece of type `PieceType`.
    pub fn new(shape_type: PieceType) -> Self {
        let shape = piece_type_to_shape(shape_type, 0);

        Piece {
            shape,
            shape_type,
            top_left: Point::new(3.0, 0.0),
            potential_top_left: Point::new(3.0, 0.0),
            shadow_position: Point::new(3.0, 0.0),
            current_rotation_index: 0,
        }
    }

    // I don't remember why I did this.
    /// Create a new piece from a reference of a `PieceType`.
    pub fn new_from_ref(shape_type: PieceType) -> Self {
        let shape = piece_type_to_shape(shape_type, 0);

        Piece {
            shape,
            shape_type,
            top_left: Point::new(3.0, 0.0),
            potential_top_left: Point::new(3.0, 0.0),
            shadow_position: Point::new(3.0, 0.0),
            current_rotation_index: 0,
        }
    }

    /// Draw the current piece.
    pub fn draw(&self, ctx: &mut Context, image: &graphics::Image) -> GameResult<()> {
        // get starting position to draw window
        // TODO: doing all of this work every frame seems bad
        let width = graphics::get_screen_coordinates(ctx).w;
        let middle = width / 2.0;
        let starting_pos = middle - ((BLOCK_SIZE as f32 * 10.0) / 2.0);

        for (r, _) in self.shape.iter().enumerate() {
            for (c, _) in self.shape[r].iter().enumerate() {
                if self.shape[r][c] != 0 {
                    if r + (self.top_left.y as usize) < 2 {
                        // don't draw in vanish zone
                        continue;
                    }
                    let colour = block_to_colour(self.shape[r][c], false);
                    graphics::set_color(ctx, colour)?;

                    let x = starting_pos + ((c as f32 + self.top_left.x as f32) * BLOCK_SIZE);
                    let y = Y_OFFSET + ((r as f32 + self.top_left.y as f32) * BLOCK_SIZE) as f32;

                    graphics::draw(ctx, image, graphics::Point::new(x, y), 0.0)?;
                }
            }
        }

        Ok(())
    }

    /// Draw the current piece's shadow.
    pub fn draw_shadow(
        &self,
        ctx: &mut Context,
        image: &graphics::Image,
        shadow_position: Point,
    ) -> GameResult<()> {
        // get starting position to draw window
        // TODO: doing all of this work every frame seems bad
        let width = graphics::get_screen_coordinates(ctx).w;
        let middle = width / 2.0;
        let starting_pos = middle - ((BLOCK_SIZE as f32 * 10.0) / 2.0);

        for (r, _) in self.shape.iter().enumerate() {
            for (c, _) in self.shape[r].iter().enumerate() {
                if self.shape[r][c] != 0 {
                    let colour = block_to_colour(self.shape[r][c], true);
                    graphics::set_color(ctx, colour)?;

                    let x = starting_pos + ((c as f32 + shadow_position.x as f32) * BLOCK_SIZE);
                    let y = Y_OFFSET + ((r as f32 + shadow_position.y as f32) * BLOCK_SIZE) as f32;

                    graphics::draw(ctx, image, graphics::Point::new(x, y), 0.0)?;
                }
            }
        }

        Ok(())
    }

    /// Draw the current piece outside of the grid at a given point.
    pub fn draw_at_point(
        &self,
        ctx: &mut Context,
        image: &graphics::Image,
        top_left: Point,
        rotation: f64,
    ) -> GameResult<()> {
        let starting_pos = top_left;
        // get the centre of our complex object in order to rotate around it
        let centre = Point::new(
            starting_pos.x + (BLOCK_SIZE * 8.0) / 2.0,
            starting_pos.y + (BLOCK_SIZE * 8.0) / 2.0,
        );

        for (r, _) in self.shape.iter().enumerate() {
            for (c, _) in self.shape[r].iter().enumerate() {
                if self.shape[r][c] != 0 {
                    let colour = block_to_colour(self.shape[r][c], false);
                    graphics::set_color(ctx, colour)?;

                    let mut x = starting_pos.x as f32 + ((c as f32 + 1.0) * BLOCK_SIZE);
                    let mut y = starting_pos.y as f32 + (r as f32 * BLOCK_SIZE);

                    let sin = rotation.sin() as f32;
                    let cos = rotation.cos() as f32;

                    // translate to origin
                    x -= centre.x;
                    y -= centre.y;

                    // rotate
                    let xnew = x * cos - y * sin;
                    let ynew = x * sin + y * cos;

                    // translate point back
                    x = xnew + centre.x;
                    y = ynew + centre.y;

                    let draw_param = DrawParam {
                        //dest: graphics::Point::new(x * scale as f32, y * scale as f32),
                        dest: graphics::Point::new(x, y),
                        rotation: rotation as f32,
                        //scale: Point::new(scale as f32, scale as f32),
                        ..Default::default()
                    };

                    graphics::draw_ex(ctx, image, draw_param)?;
                }
            }
        }

        // TODO: global debug flag to control whether this appears or not,
        // should be able to toggle via an environmental variable for ease-of-use

        // graphics::set_color(ctx, Color::new(255.0, 0.0, 0.0, 255.0))?;
        // graphics::circle(ctx, graphics::DrawMode::Fill, centre, 2.0, 22)?;
        // graphics::set_color(ctx, Color::new(0.0, 0.0, 255.0, 255.0))?;
        // graphics::circle(ctx, graphics::DrawMode::Fill, top_left, 2.0, 22)?;

        Ok(())
    }

    /// Return what the next shape of the current piece would be given a
    /// particular rotation `direction` (-1 for left, 1 for right)
    pub fn get_next_shape(&self, direction: i32) -> PieceShape {
        let next_index = next_rotation_index(self.current_rotation_index, direction);

        piece_type_to_shape(self.shape_type, next_index as usize)
    }

    /// Returns the current piece's shape.
    pub fn get_shape(&self) -> PieceShape {
        self.shape
    }

    /// Returns the current piece's type.
    pub fn get_type(&self) -> PieceType {
        self.shape_type
    }

    /// Given a direction, change the current piece's shape.
    pub fn change_shape(&mut self, direction: i32) {
        let next_index = next_rotation_index(self.current_rotation_index, direction);

        self.shape = piece_type_to_shape(self.shape_type, next_index as usize);
        self.current_rotation_index = next_index;
    }

    /// Set the current piece's shadow position.
    pub fn set_shadow_position(&mut self, shadow_pos: Point) {
        self.shadow_position = shadow_pos;
    }

    /// Get the current piece's shadow position.
    pub fn get_shadow_position(&self) -> Point {
        self.shadow_position
    }
}

/*
 * Utility methods
*/

/// Given a type of piece, return the shape at the given rotation index.
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

/// Given an integer that represents a type of piece, return the `PieceType`.
pub fn u8_to_piece_type(num: u8) -> Option<PieceType> {
    match num {
        0 => Some(PieceType::I),
        1 => Some(PieceType::J),
        2 => Some(PieceType::L),
        3 => Some(PieceType::O),
        4 => Some(PieceType::S),
        5 => Some(PieceType::T),
        6 => Some(PieceType::Z),
        _ => unreachable!(),
    }
}

/// Get the block colour depending on the shape. If `shadow` is true, the
/// colour's alpha channel is reduced.
pub fn block_to_colour(num: u32, shadow: bool) -> Color {
    if shadow {
        match num {
            1 => Color::from((0, 255, 255, 75)),
            2 => Color::from((0, 0, 255, 75)),
            3 => Color::from((255, 165, 0, 75)),
            4 => Color::from((255, 255, 0, 75)),
            5 => Color::from((128, 255, 0, 75)),
            6 => Color::from((128, 0, 128, 75)),
            7 => Color::from((255, 0, 0, 75)),
            _ => unreachable!(),
        }
    } else {
        match num {
            1 => Color::from((0, 255, 255, 255)),
            2 => Color::from((0, 0, 255, 255)),
            3 => Color::from((255, 165, 0, 255)),
            4 => Color::from((255, 255, 0, 255)),
            5 => Color::from((128, 255, 0, 255)),
            6 => Color::from((128, 0, 128, 255)),
            7 => Color::from((255, 0, 0, 255)),
            _ => unreachable!(),
        }
    }
}

/// Return the index of the next rotation shape, given the current rotation
/// shape's index and a direction.
pub fn next_rotation_index(current_index: u32, direction: i32) -> u32 {
    if direction == -1 {
        match current_index {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            _ => unreachable!(),
        }
    } else {
        match current_index {
            0 => 3,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!(),
        }
    }
}
