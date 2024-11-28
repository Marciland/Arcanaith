pub(crate) struct VisualComponent {
    texture_indices: Vec<usize>,
    current_texture: usize,
    layer: Layer,
    frame_duration: usize,
    current_frame: usize,
    visible: bool,
}

impl VisualComponent {
    fn new(texture_indices: Vec<usize>, layer: Layer, frame_duration: usize) -> Self {
        Self {
            texture_indices,
            current_texture: 0,
            layer,
            frame_duration,
            current_frame: 0,
            visible: true,
        }
    }

    fn update_animation(&mut self) {
        if self.frame_duration == 0 || self.texture_indices.len() == 1 {
            return;
        }

        self.current_frame += 1;
        if self.current_frame >= self.frame_duration {
            self.current_frame = 0;
            self.current_texture = (self.current_texture + 1) % self.texture_indices.len();
        }
    }

    fn get_current_texture(&self) -> usize {
        self.texture_indices[self.current_texture]
    }

    fn should_render(&self) -> bool {
        self.visible
    }
}

pub(crate) enum Layer {
    Interface,
    Game,
    Background,
}

impl Layer {
    fn value(&self) -> u8 {
        match self {
            Layer::Interface => 0,
            Layer::Game => 1,
            Layer::Background => 2,
        }
    }
}
