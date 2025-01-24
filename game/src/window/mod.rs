use rendering::{RenderAPI, RenderContext};
use winit::{
    event_loop::ActiveEventLoop,
    window::{Fullscreen::Borderless, Icon},
};

use crate::constants::{FRAGSHADER, FRAMES_IN_FLIGHT, FULLSCREEN, ICONPATH, TITLE, VERTSHADER};

pub struct Window<API: RenderAPI> {
    inner_window: winit::window::Window,
    pub render_context: RenderContext<API>,
}

impl<API: RenderAPI> Window<API> {
    pub fn create(event_loop: &ActiveEventLoop, max_texture_count: u32) -> Self {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(ICONPATH)
                .expect("Failed to open icon image!")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon =
            Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to create icon!");

        let mut attributes = winit::window::Window::default_attributes()
            .with_title(TITLE)
            .with_window_icon(Some(icon))
            .with_visible(false);
        if FULLSCREEN {
            attributes = attributes.with_fullscreen(Some(Borderless(None)));
        }

        let inner_window = event_loop
            .create_window(attributes)
            .expect("Failed to create inner window!");

        let render_context = RenderContext::create(
            &inner_window,
            max_texture_count,
            TITLE,
            FRAMES_IN_FLIGHT,
            VERTSHADER,
            FRAGSHADER,
        );

        inner_window.set_visible(true);
        Self {
            inner_window,
            render_context,
        }
    }

    pub fn is_minimized(&self) -> Option<bool> {
        self.inner_window.is_minimized()
    }

    pub fn request_render(&self) {
        self.inner_window.request_redraw();
    }
}
