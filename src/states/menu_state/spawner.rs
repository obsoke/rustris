use std::time::Duration;
use rand;
use ggez::Context;
use ggez::graphics::Point;
use event::Assets;
use states::play_state::tetromino::{Piece, PieceType, u8_to_piece_type};
use util::DurationExt;

const SPEED: f32 = 170.0;

struct SpawnedPiece {
    position: Point,
    extra_rotation: f64,
    scale: f64,
    piece: Piece,
    is_dead: bool,
}

impl SpawnedPiece {
    pub fn new(pos: Point, rot: f64, scale: f64, ptype: PieceType) -> Self {
        Self {
            position: pos,
            extra_rotation: rot,
            scale: scale,
            piece: Piece::new(ptype),
            is_dead: false,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context, _assets: &Assets, dt: Duration) {
        // TODO: update position
        self.position.y += SPEED * dt.as_subsec_millis() as f32;

        if self.position.y >= 720.0 {
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

// const TIME_BETWEEN_SPAWNS: f64 = 0.5;
const TIME_BETWEEN_SPAWNS: f64 = 2.5;

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
            self.spawn_new_piece();
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

    fn spawn_new_piece(&mut self) {
        use rand::Rng;
        let x = rand::thread_rng().gen_range(0.0, 1024.0);
        let extra_rotation = rand::thread_rng().gen_range(0.0, 50.0);
        let piece_type = {
            let piece_type_no = rand::thread_rng().gen_range(0, 7);
            u8_to_piece_type(piece_type_no).unwrap()
        };
        // TODO: add new piece to 'active_pieces' give it a random X position,
        // random extra rotation
        let position = Point::new(x, 0.0);
        self.active_pieces.push(SpawnedPiece::new(
            position,
            extra_rotation,
            1.0,
            piece_type,
        ));
    }
}
