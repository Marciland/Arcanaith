mod overlay;

use crate::{
    objects::{Object, Player},
    ECS,
};
use ash::Device;
use overlay::Overlay;

pub struct Game {
    pub objects: Vec<Object>,
}

impl Game {
    pub fn create(ecs: &mut ECS) -> Game {
        let mut objects = Vec::with_capacity(100);

        let overlay = Overlay::create(ecs);
        objects.extend(overlay.objects);

        // TODO background

        let player = Player::create(ecs);
        objects.push(Object::Player(player));

        // TODO spawner

        Game { objects }
    }

    pub fn get_player(&self) -> Option<&Player> {
        for obj in &self.objects {
            if let Object::Player(player) = obj {
                return Some(player);
            }
        }

        None
    }

    pub fn destroy(&self, device: &Device, ecs: &mut ECS) {
        for obj in &self.objects {
            ecs.destroy_entity(obj.id(), device);
        }
    }
}
