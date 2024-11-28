use super::{InputComponent, Layer, PositionComponent, TextComponent, VisualComponent};

 struct TextWithPosition<'component> {
     text: &'component mut TextComponent,
     position: &'component PositionComponent,
}

 struct VisualWithPosition<'component> {
     visual: &'component VisualComponent,
     position: &'component PositionComponent,
}

 struct InputWithPosition<'component> {
     input: &'component InputComponent,
     position: &'component PositionComponent,
}

 enum RenderTarget<'component> {
    Visual(VisualWithPosition<'component>),
    Text(TextWithPosition<'component>),
}

impl<'component> RenderTarget<'component> {
     fn get_layer(&self) -> &Layer {
        match self {
            RenderTarget::Visual(v) => &v.visual.layer,
            RenderTarget::Text(t) => &t.text.layer,
        }
    }

     fn get_position(&self) -> &PositionComponent {
        match self {
            RenderTarget::Visual(v) => v.position,
            RenderTarget::Text(t) => t.position,
        }
    }
}
