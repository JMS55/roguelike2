mod ecs;

use crate::ecs::{PlayerAI, Position, Sprite, WanderAI, WorldExtension};
use hecs::{Entity, World};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rltk::{GameState, Rltk, RltkBuilder, RltkError, VirtualKeyCode, INPUT};
use std::time::{Duration, Instant};

struct Game {
    world: World,
    player_entity: Entity,
    rng: ThreadRng,
    last_update: Instant,
    update_counter: u32,
}

impl Game {
    fn new() -> Self {
        let mut world = World::new();
        let player_entity = world.spawn((
            PlayerAI,
            Position { x: 40, y: 25 },
            Sprite {
                character: '@',
                color: (255, 255, 255),
            },
        ));
        for i in 0..10 {
            world.spawn((
                WanderAI,
                Position { x: i, y: 25 },
                Sprite {
                    character: 'E',
                    color: (230, 230, 230),
                },
            ));
        }

        Self {
            world,
            player_entity,
            rng: thread_rng(),
            last_update: Instant::now(),
            update_counter: 0,
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Update
        if self.last_update.elapsed() >= Duration::from_nanos(33333330) {
            self.last_update = Instant::now();
            self.update_counter += 1;

            // PlayerAI
            let input = INPUT.lock();
            let mut x = 0;
            let mut y = 0;
            if input.is_key_pressed(VirtualKeyCode::W) {
                y = -1;
            }
            if input.is_key_pressed(VirtualKeyCode::A) {
                x = -1;
            }
            if input.is_key_pressed(VirtualKeyCode::S) {
                y = 1;
            }
            if input.is_key_pressed(VirtualKeyCode::D) {
                x = 1;
            }
            if x != 0 && y != 0 {
                if self.update_counter % 4 < 2 {
                    y = 0;
                } else {
                    x = 0;
                }
            }
            self.world.try_move_entity(self.player_entity, x, y);

            // WanderAI
            let entities = self
                .world
                .query::<(&WanderAI, &Position)>()
                .iter()
                .map(|(entity, (_, _))| entity)
                .collect::<Vec<Entity>>();
            for entity in entities {
                let mut x = self.rng.gen_range(-1, 2);
                let mut y = self.rng.gen_range(-1, 2);
                if x != 0 && y != 0 {
                    if self.update_counter % 4 < 2 {
                        y = 0;
                    } else {
                        x = 0;
                    }
                }
                self.world.try_move_entity(entity, x, y);
            }
        }

        // Render
        ctx.cls();
        for (_, (position, sprite)) in self.world.query::<(&Position, &Sprite)>().iter() {
            ctx.set(
                position.x,
                position.y,
                sprite.color,
                (0, 0, 0),
                rltk::to_cp437(sprite.character),
            );
        }
    }
}

fn main() -> RltkError {
    let mut terminal = RltkBuilder::simple80x50()
        .with_title("roguelike2")
        .with_fps_cap(60.0)
        .build()?;
    terminal.post_scanlines = true;
    rltk::main_loop(terminal, Game::new())
}
