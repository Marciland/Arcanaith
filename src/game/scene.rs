use super::{object::Object, sprites::load_sprite_from_file, GameObject, Quad};
use crate::{ModelViewProjection, UniformBufferObject, Window};
use ash::{
    vk::{
        Buffer, DescriptorPool, DescriptorSet, DescriptorSetLayout, ImageView, Pipeline,
        PipelineLayout, RenderPass, Sampler,
    },
    Device,
};
use glam::Vec2;
use std::{cell::RefCell, mem::size_of, rc::Rc};

pub struct Scene {
    _window: Rc<RefCell<Window>>,
    render_pass: RenderPass,
    texture_sampler: Sampler,
    descriptor_set_layout: DescriptorSetLayout,
    descriptor_pool: DescriptorPool,
    descriptor_set: DescriptorSet,
    pipeline: Pipeline,
    pipeline_layout: PipelineLayout,
    mvp_uniform: UniformBufferObject,
    objects_uniform: UniformBufferObject,
    quad: Quad,
    objects: Vec<Object>,
}

impl Scene {
    pub fn create(window: Rc<RefCell<Window>>) -> Self {
        let quad = Quad::create(window.borrow());
        let background = Object::create(
            ModelViewProjection::default().scale(Vec2 { x: 2.0, y: 2.0 }),
            vec![window.borrow().create_texture(
                image::open("res/main_menu/background.png")
                    .unwrap()
                    .into_rgba8(),
            )],
            Layer::Background,
        );

        let player_textures =
            load_sprite_from_file(window.borrow(), "res/sprites/player_walking.png", 4, 4);
        let player = Object::create(ModelViewProjection::default(), player_textures, Layer::Game);

        let interface = Object::create(
            ModelViewProjection::default()
                .translate(Vec2 { x: -0.5, y: 0.5 })
                .scale(Vec2 { x: 0.5, y: 0.5 }),
            vec![window
                .borrow()
                .create_texture(image::open("res/icon.png").unwrap().into_rgba8())],
            Layer::Interface,
        );

        let mut objects = vec![background, player, interface];
        objects.sort_by_key(|obj| obj.get_depth());
        let object_count = objects.len();

        let mut texture_image_views: Vec<ImageView> = Vec::with_capacity(object_count);
        for obj in &objects {
            let initial_texture = &obj.get_textures()[obj.texture_index];
            texture_image_views.push(initial_texture.get_view())
        }

        let (
            render_pass,
            texture_sampler,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            pipeline_layout,
            pipeline,
            mvp_uniform,
            objects_uniform,
        ) = {
            let _window = window.borrow();

            let mvp_uniform_size = size_of::<ModelViewProjection>() * object_count;
            let objects_uniform_size = size_of::<i32>();
            let mvp_uniform = _window.create_uniform_buffer(mvp_uniform_size as u64);
            let objects_uniform = _window.create_uniform_buffer(objects_uniform_size as u64);

            let render_pass = _window.create_render_pass();
            let texture_sampler = _window.create_texture_sampler();
            let (descriptor_set_layout, descriptor_pool) =
                _window.create_descriptor_pool(object_count as u32);
            let descriptor_set = _window.create_descriptor_set(
                descriptor_pool,
                descriptor_set_layout,
                &texture_image_views,
                texture_sampler,
                object_count,
                objects_uniform.get_buffer(),
                mvp_uniform.get_buffer(),
            );
            let (pipeline_layout, pipeline) =
                _window.create_pipeline(render_pass, &[descriptor_set_layout]);

            (
                render_pass,
                texture_sampler,
                descriptor_set_layout,
                descriptor_pool,
                descriptor_set,
                pipeline_layout,
                pipeline,
                mvp_uniform,
                objects_uniform,
            )
        };

        Self {
            _window: window,
            render_pass,
            texture_sampler,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            pipeline,
            pipeline_layout,
            mvp_uniform,
            objects_uniform,
            quad,
            objects,
        }
    }

    pub fn get_vertex_buffer(&self) -> Buffer {
        self.quad.get_vertex_buffer()
    }

    pub fn get_index_buffer(&self) -> Buffer {
        self.quad.get_index_buffer()
    }

    pub fn get_render_pass(&self) -> RenderPass {
        self.render_pass
    }

    pub fn get_pipeline(&self) -> Pipeline {
        self.pipeline
    }

    pub fn get_pipeline_layout(&self) -> PipelineLayout {
        self.pipeline_layout
    }

    pub fn get_index_count(&self) -> u32 {
        self.quad.get_index_count()
    }

    pub fn get_objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn get_mvp_uniform(&self) -> &UniformBufferObject {
        &self.mvp_uniform
    }

    pub fn get_objects_uniform(&self) -> &UniformBufferObject {
        &self.objects_uniform
    }

    pub fn get_descriptor_set(&self) -> DescriptorSet {
        self.descriptor_set
    }

    pub fn get_mvp_data(&self) -> Vec<ModelViewProjection> {
        let mut mvp_data: Vec<ModelViewProjection> = Vec::with_capacity(self.objects.len());
        for object in &self.objects {
            mvp_data.push(*object.get_mvp());
        }
        mvp_data
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        device.destroy_pipeline(self.pipeline, None);
        device.destroy_pipeline_layout(self.pipeline_layout, None);

        device.destroy_descriptor_pool(self.descriptor_pool, None);
        device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

        self.mvp_uniform.destroy(device);
        self.objects_uniform.destroy(device);

        device.destroy_sampler(self.texture_sampler, None);

        for obj in &self.objects {
            obj.destroy(device);
        }

        self.quad.destroy(device);

        device.destroy_render_pass(self.render_pass, None);
    }
}

#[derive(Copy, Clone)]
pub enum Layer {
    Interface,
    Game,
    Background,
}

impl Layer {
    pub fn value(&self) -> u8 {
        match self {
            Layer::Interface => 0,
            Layer::Game => 1,
            Layer::Background => 2,
        }
    }
}
