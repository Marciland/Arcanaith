use crate::ecs::component::{InputComponent, PositionComponent, TextComponent, VisualComponent};

pub struct TextWithPosition<'component> {
    pub text: &'component mut TextComponent,
    pub position: &'component PositionComponent,
}

pub struct VisualWithPosition<'component> {
    pub visual: &'component mut VisualComponent,
    pub position: &'component PositionComponent,
}

pub struct InputWithPosition<'component> {
    pub input: &'component InputComponent,
    pub position: &'component PositionComponent,
}
