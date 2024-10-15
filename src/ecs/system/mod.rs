use ash::Device;
pub use render::RenderSystem;
pub use resource::ResourceSystem;
mod render;
mod resource;

pub struct SystemManager {
    pub render_system: RenderSystem,
    pub resource_system: ResourceSystem,
}

impl SystemManager {
    pub fn create() -> Self {
        Self {
            render_system: RenderSystem::create(),
            resource_system: ResourceSystem::create(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        self.render_system.destroy(device);
        self.resource_system.destroy(device);
    }
}
