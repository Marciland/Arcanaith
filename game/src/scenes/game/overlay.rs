use crate::GameEvent;

use ecs::{Entity, Layer, TextContent, ECS};
use glam::Vec2;
use objects::{Content, Factory};
use winit::event_loop::EventLoopProxy;

pub struct Overlay {
    pub objects: Vec<Entity>,
}

impl Overlay {
    pub fn create(ecs: &mut ECS<GameEvent>) -> Self {
        let mut objects: Vec<Entity> = Vec::with_capacity(8);

        objects.push(Overlay::create_health_bar(ecs));
        objects.push(Overlay::create_mana_bar(ecs));
        objects.push(Overlay::create_exp_bar(ecs));
        objects.push(Overlay::create_money_bag(ecs));
        objects.push(Overlay::create_inventory(ecs));
        objects.push(Overlay::create_wave_counter(ecs));
        objects.push(Overlay::create_highscore(ecs));
        objects.push(Overlay::create_pause(ecs));

        Self { objects }
    }

    fn create_health_bar(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::status_bar(
            ecs,
            Vec2 {
                x: -0.925,
                y: 0.925,
            },
            Vec2 { x: 0.15, y: 0.05 },
        )
    }

    fn create_mana_bar(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::status_bar(
            ecs,
            Vec2 {
                x: -0.925,
                y: 0.975,
            },
            Vec2 { x: 0.15, y: 0.05 },
        )
    }

    fn create_exp_bar(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::status_bar(ecs, Vec2 { x: 0.0, y: 0.975 }, Vec2 { x: 0.6, y: 0.05 })
    }

    fn create_money_bag(ecs: &mut ECS<GameEvent>) -> Entity {
        ecs.new_icon_text(
            Vec2 { x: 0.7, y: 0.975 },
            Vec2 { x: 0.1, y: 0.05 },
            "money_bag",
            TextContent {
                text: "0".to_owned(),
                font: "test".to_owned(), // TODO adjust font
                font_size: 50.0,         // TODO adjust font size
            },
        )
    }

    fn create_inventory(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::button(
            ecs,
            Vec2 { x: 0.85, y: 0.975 },
            Vec2 { x: 0.1, y: 0.1 },
            Content::Image {
                name: "bag",
                layer: Layer::Interface,
            },
            false,
            open_inventory,
        )
    }

    fn create_wave_counter(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::label(
            ecs,
            Vec2 { x: 0.0, y: -0.8 },
            Vec2 { x: 0.6, y: 0.1 },
            Content::Text(TextContent {
                text: "Waves".to_owned(),
                font: "test".to_owned(), // TODO adjust font
                font_size: 50.0,         // TODO adjust font size
            }),
        )
    }

    fn create_highscore(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::label(
            ecs,
            Vec2 { x: 0.0, y: -0.9 },
            Vec2 { x: 0.6, y: 0.1 },
            Content::Text(TextContent {
                text: "Highscore".to_owned(),
                font: "test".to_owned(), // TODO adjust font
                font_size: 50.0,         // TODO adjust font size
            }),
        )
    }

    fn create_pause(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::button(
            ecs,
            Vec2 {
                x: -0.925,
                y: -0.925,
            },
            Vec2 { x: 0.05, y: 0.05 },
            Content::Image {
                name: "pause_button",
                layer: Layer::Interface,
            },
            false,
            pause_clicked,
        )
    }
}

fn pause_clicked(_event_proxy: &EventLoopProxy<GameEvent>) {
    todo!("pause clicked")
}

fn open_inventory(_event_proxy: &EventLoopProxy<GameEvent>) {
    todo!("open inventory")
}
