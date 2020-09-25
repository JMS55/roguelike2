use hecs::{Entity, World};

pub trait WorldExtension {
    // Does nothing if entity does not exist, does not have a Position component, or cannot move in the specified direction.
    fn try_move_entity(&self, entity: Entity, x: i32, y: i32);
}

impl WorldExtension for World {
    fn try_move_entity(&self, entity: Entity, x: i32, y: i32) {
        let entity_position = self
            .query_one::<&Position>(entity)
            .ok()
            .and_then(|mut query| query.get().cloned());
        if let Some(entity_position) = entity_position {
            let new_position = Position {
                x: entity_position.x + x,
                y: entity_position.y + y,
            };
            if !(0..80).contains(&new_position.x) || !(0..50).contains(&new_position.y) {
                return;
            }
            for (other_entity, position) in self.query::<&Position>().iter() {
                if other_entity != entity && position == &new_position {
                    return;
                }
            }
            *self
                .query_one::<&mut Position>(entity)
                .unwrap()
                .get()
                .unwrap() = new_position;
        }
    }
}

pub struct PlayerAI;

pub struct WanderAI;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Sprite {
    pub character: char,
    pub color: (u8, u8, u8),
}
