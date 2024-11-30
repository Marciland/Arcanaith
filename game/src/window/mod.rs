mod renderer;

use ash::vk::Extent2D;
use renderer::Renderer;

pub struct Window {
    inner: winit::window::Window,
    renderer: Renderer,
}

impl Window {
    pub fn create(inner_window: winit::window::Window, max_texture_count: u32) -> Self {
        let renderer = Renderer::create(&inner_window, max_texture_count);

        inner_window.set_visible(true);
        Self {
            inner: inner_window,
            renderer,
        }
    }

    pub fn is_minimized(&self) -> Option<bool> {
        self.inner.is_minimized()
    }

    pub fn request_render(&self) {
        self.inner.request_redraw();
    }

    pub fn get_current_size(&self) -> Extent2D {
        self.renderer.get_extent()
    }

    pub fn get_render_context(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
