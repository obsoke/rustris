use std::time::Duration;
use rand;
use ggez::{Context, graphics};
use ggez::graphics::Point;
use crate::event::Assets;
use crate::states::play_state::tetromino::{Piece, PieceType, u8_to_piece_type};
use crate::util::DurationExt;

const SPEED: f32 = 170.0;

struct SpawnedPiece {
    position: Point,
    extra_rotation: f64,
    piece: Piece,
    is_dead: bool,
}

impl SpawnedPiece {
    pub fn new(pos: Point, rot: f64, ptype: PieceType) -> Self {
        Self {
            position: pos,
            extra_rotation: rot,
            piece: Piece::new(ptype),
            is_dead: false,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, _assets: &Assets, dt: Duration) {
        // TODO: update position
        let coords = graphics::get_screen_coordinates(ctx);
        let height = coords.h * -1.0;
        self.position.y += SPEED * dt.as_subsec_millis() as f32;

        if self.position.y >= height {
            self.is_dead = true;
        }
        self.extra_rotation += dt.as_subsec_millis();
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) {
        self.piece
            .draw_at_point(
                ctx,
                assets.get_image("block").unwrap(),
                self.position,
                self.extra_rotation,
            )
            .unwrap();
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }
}

const TIME_BETWEEN_SPAWNS: f64 = 0.5;
const MAX_ROTATION: f64 = 50.0;

pub struct Spawner {
    active_pieces: Vec<SpawnedPiece>,
    time_until_next_spawn: f64,
}

impl Spawner {
    pub fn new() -> Self {
        Self {
            active_pieces: vec![],
            time_until_next_spawn: 0.0,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration) {
        // update current time
        self.time_until_next_spawn += dt.as_subsec_millis();
        if self.time_until_next_spawn >= TIME_BETWEEN_SPAWNS {
            self.time_until_next_spawn -= TIME_BETWEEN_SPAWNS;
            self.spawn_new_piece(ctx);
        }

        // check for dead pieces
        for i in (0..self.active_pieces.len()).rev() {
            if self.active_pieces[i].is_dead() {
                self.active_pieces.remove(i);
            }
        }

        // check if time to spawn new piece
        for piece in &mut self.active_pieces {
            piece.update(ctx, assets, dt);
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) {
        for piece in &self.active_pieces {
            piece.draw(ctx, assets);
        }
    }

    fn spawn_new_piece(&mut self, ctx: &Context) {
        use rand::Rng;
        let coords = graphics::get_screen_coordinates(ctx);
        let x = rand::thread_rng().gen_range(0.0, coords.w);
        let extra_rotation = rand::thread_rng().gen_range(0.0, MAX_ROTATION);

        let piece_type = {
            let piece_type_no = rand::thread_rng().gen_range(0, 7);
            u8_to_piece_type(piece_type_no).unwrap()
        };

        // spawn at a random x position way above viewable screen so pieces just
        // dont 'pop' in but kinda just float into view
        let position = Point::new(x, -200.0);
        self.active_pieces.push(SpawnedPiece::new(
            position,
            extra_rotation,
            piece_type,
        ));
    }
}
