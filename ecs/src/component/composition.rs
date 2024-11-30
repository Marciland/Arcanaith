use super::{Layer, PositionComponent, TextComponent, VisualComponent};

pub struct TextWithPosition<'component> {
    pub text: &'component mut TextComponent,
    pub position: &'component PositionComponent,
}

pub struct VisualWithPosition<'component> {
    pub visual: &'component VisualComponent,
    pub position: &'component PositionComponent,
}

pub enum RenderTarget<'component> {
    Visual(VisualWithPosition<'component>),
    Text(TextWithPosition<'component>),
}

impl<'component> RenderTarget<'component> {
    pub fn get_layer(&self) -> &Layer {
        match self {
            RenderTarget::Visual(v) => &v.visual.layer,
            RenderTarget::Text(t) => &t.text.layer,
        }
    }

    pub fn get_position(&self) -> &PositionComponent {
        match self {
            RenderTarget::Visual(v) => v.position,
            RenderTarget::Text(t) => t.position,
        }
    }
}
