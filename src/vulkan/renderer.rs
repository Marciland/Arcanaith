use ash::{
    vk::{
        BorderColor, Buffer, BufferUsageFlags, ClearColorValue, ClearDepthStencilValue, ClearValue,
        CommandBuffer, CommandBufferBeginInfo, CommandPool, CompareOp, DescriptorBufferInfo,
        DescriptorImageInfo, DescriptorPool, DescriptorPoolCreateInfo, DescriptorPoolSize,
        DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorType,
        DeviceMemory, Extent2D, Filter, Format, Framebuffer, Image, ImageAspectFlags, ImageLayout,
        ImageTiling, ImageUsageFlags, ImageView, IndexType, MemoryMapFlags, MemoryPropertyFlags,
        Offset2D, PhysicalDevice, Pipeline, PipelineBindPoint, PipelineLayout, Queue, Rect2D,
        RenderPass, RenderPassBeginInfo, Sampler, SamplerAddressMode, SamplerCreateInfo,
        SamplerMipmapMode, SubpassContents, Viewport, WriteDescriptorSet,
    },
    Device, Instance,
};
use glam::{Mat4, Vec3};
use std::{ffi::c_void, mem, ptr::copy_nonoverlapping};

use crate::shader_structs::{UniformBufferObject, Vertex};

use super::{internal::InternalVulkan, VulkanWrapper};

pub trait VulkanRenderer {
    #[allow(clippy::too_many_arguments)]
    unsafe fn begin_render_pass(
        device: &Device,
        render_pass: RenderPass,
        framebuffers: &[Framebuffer],
        vertex_buffers: &[Buffer],
        index_buffer: Buffer,
        image_index: usize,
        command_buffer: CommandBuffer,
        pipeline: Pipeline,
        extent: Extent2D,
        index_count: u32,
        pipeline_layout: PipelineLayout,
        descriptor_sets: &[DescriptorSet],
    );
    unsafe fn update_uniform_buffer(extent: Extent2D, mapped: *mut c_void);
    unsafe fn create_vertex_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        vertices: &[Vertex],
        command_pool: CommandPool,
        graphics_queue: Queue,
    ) -> (Buffer, DeviceMemory);
    unsafe fn create_index_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        indices: &[u16],
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Buffer, DeviceMemory);
    unsafe fn create_uniform_buffers(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
    ) -> (Vec<Buffer>, Vec<DeviceMemory>, Vec<*mut c_void>);
    unsafe fn create_descriptors(
        device: &Device,
        descriptor_set_layout: DescriptorSetLayout,
        uniform_buffers: &[Buffer],
        texture_image_view: ImageView,
        texture_sampler: Sampler,
    ) -> (DescriptorPool, DescriptorSet);
    unsafe fn create_texture_image(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Image, DeviceMemory, ImageView);
    unsafe fn create_texture_sampler(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
    ) -> Sampler;
}

impl VulkanRenderer for VulkanWrapper {
    unsafe fn begin_render_pass(
        device: &Device,
        render_pass: RenderPass,
        framebuffers: &[Framebuffer],
        vertex_buffers: &[Buffer],
        index_buffer: Buffer,
        image_index: usize,
        command_buffer: CommandBuffer,
        pipeline: Pipeline,
        extent: Extent2D,
        index_count: u32,
        pipeline_layout: PipelineLayout,
        descriptor_sets: &[DescriptorSet],
    ) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_command_buffer_recording
        let begin_info = CommandBufferBeginInfo::default();

        device
            .begin_command_buffer(command_buffer, &begin_info)
            .unwrap();

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_starting_a_render_pass
        let clear_colors = [
            ClearValue {
                color: ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            },
            ClearValue {
                depth_stencil: ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];

        let render_pass_begin_info = RenderPassBeginInfo::default()
            .render_pass(render_pass)
            .framebuffer(framebuffers[image_index])
            .render_area(Rect2D {
                offset: Offset2D { x: 0, y: 0 },
                extent,
            })
            .clear_values(&clear_colors);

        device.cmd_begin_render_pass(
            command_buffer,
            &render_pass_begin_info,
            SubpassContents::INLINE,
        );

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_basic_drawing_commands
        device.cmd_bind_pipeline(command_buffer, PipelineBindPoint::GRAPHICS, pipeline);

        device.cmd_bind_vertex_buffers(command_buffer, 0, vertex_buffers, &[0]);
        device.cmd_bind_index_buffer(command_buffer, index_buffer, 0, IndexType::UINT16);

        let viewports = [Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(extent.width as f32)
            .height(extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)];

        device.cmd_set_viewport(command_buffer, 0, &viewports);

        let scissors = [Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent,
        }];

        device.cmd_set_scissor(command_buffer, 0, &scissors);

        device.cmd_bind_descriptor_sets(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            pipeline_layout,
            0,
            descriptor_sets,
            &[],
        );

        device.cmd_draw_indexed(command_buffer, index_count, 1, 0, 0, 0);

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_finishing_up
        device.cmd_end_render_pass(command_buffer);
        device.end_command_buffer(command_buffer).unwrap();
    }

    unsafe fn update_uniform_buffer(extent: Extent2D, mapped: *mut c_void) {
        // https://docs.vulkan.org/tutorial/latest/05_Uniform_buffers/00_Descriptor_set_layout_and_buffer.html#_updating_uniform_data
        let mut ubo = UniformBufferObject {
            _model: Mat4::IDENTITY,
            _view: Mat4::look_at_rh(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 2.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
            ),
            projection: Mat4::perspective_rh(
                60.0_f32.to_radians(),
                extent.width as f32 / extent.height as f32,
                0.1,
                10.0,
            ),
        };
        ubo.projection.y_axis.y *= -1.0;

        copy_nonoverlapping(
            &ubo as *const UniformBufferObject,
            mapped as *mut UniformBufferObject,
            1,
        )
    }

    unsafe fn create_vertex_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        vertices: &[Vertex],
        command_pool: CommandPool,
        graphics_queue: Queue,
    ) -> (Buffer, DeviceMemory) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/01_Vertex_buffer_creation.html
        let buffer_size = mem::size_of_val(vertices) as u64;

        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/02_Staging_buffer.html#_using_a_staging_buffer
        let (staging_buffer, staging_buffer_memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );

        let data = device
            .map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                MemoryMapFlags::empty(),
            )
            .unwrap();
        copy_nonoverlapping(vertices.as_ptr(), data as *mut Vertex, vertices.len());
        device.unmap_memory(staging_buffer_memory);

        let (vertex_buffer, vertex_buffer_memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::VERTEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        InternalVulkan::copy_buffer(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            vertex_buffer,
            buffer_size,
        );
        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);

        (vertex_buffer, vertex_buffer_memory)
    }

    unsafe fn create_index_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        indices: &[u16],
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Buffer, DeviceMemory) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/03_Index_buffer.html
        let buffer_size = mem::size_of_val(indices) as u64;

        let (staging_buffer, staging_buffer_memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );

        let data = device
            .map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                MemoryMapFlags::empty(),
            )
            .unwrap();
        copy_nonoverlapping(indices.as_ptr(), data as *mut u16, indices.len());
        device.unmap_memory(staging_buffer_memory);

        let (index_buffer, index_buffer_memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::INDEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        InternalVulkan::copy_buffer(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            index_buffer,
            buffer_size,
        );
        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);

        (index_buffer, index_buffer_memory)
    }

    unsafe fn create_uniform_buffers(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
    ) -> (Vec<Buffer>, Vec<DeviceMemory>, Vec<*mut c_void>) {
        // https://docs.vulkan.org/tutorial/latest/05_Uniform_buffers/00_Descriptor_set_layout_and_buffer.html#_uniform_buffer
        let buffer_size = mem::size_of::<UniformBufferObject>() as u64;

        let (buffer, memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::UNIFORM_BUFFER,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );
        let mapped = device
            .map_memory(memory, 0, buffer_size, MemoryMapFlags::empty())
            .unwrap();

        (vec![buffer], vec![memory], vec![mapped])
    }

    unsafe fn create_descriptors(
        device: &Device,
        descriptor_set_layout: DescriptorSetLayout,
        uniform_buffers: &[Buffer],
        texture_image_view: ImageView,
        texture_sampler: Sampler,
    ) -> (DescriptorPool, DescriptorSet) {
        // https://docs.vulkan.org/tutorial/latest/05_Uniform_buffers/01_Descriptor_pool_and_sets.html#_introduction
        let pool_sizes = [
            DescriptorPoolSize::default()
                .descriptor_count(1)
                .ty(DescriptorType::UNIFORM_BUFFER),
            DescriptorPoolSize::default()
                .descriptor_count(1)
                .ty(DescriptorType::COMBINED_IMAGE_SAMPLER),
        ];
        let pool_info = DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(1);

        let descriptor_pool = device.create_descriptor_pool(&pool_info, None).unwrap();

        let descriptor_set_layouts = [descriptor_set_layout];

        let descriptor_allocation_info = DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&descriptor_set_layouts);

        let descriptor_set = device
            .allocate_descriptor_sets(&descriptor_allocation_info)
            .unwrap()[0];

        let buffer_infos = [DescriptorBufferInfo::default()
            .buffer(uniform_buffers[0])
            .offset(0)
            .range(mem::size_of::<UniformBufferObject>() as u64)];

        let image_infos = [DescriptorImageInfo::default()
            .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view(texture_image_view)
            .sampler(texture_sampler)];

        let descriptor_writes = [
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .buffer_info(&buffer_infos),
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .image_info(&image_infos),
        ];

        device.update_descriptor_sets(&descriptor_writes, &[]);

        (descriptor_pool, descriptor_set)
    }

    unsafe fn create_texture_image(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Image, DeviceMemory, ImageView) {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_loading_an_image
        // let image = image::open("res/sprites/player_walking.png")
        let image = image::open(crate::constants::ICONPATH)
            .unwrap()
            .into_rgba8(); // use as arg
        let (width, height) = image.dimensions();
        let image_size = (width * height * 4) as u64;
        let image_extent = Extent2D { width, height };

        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_staging_buffer
        let (staging_buffer, staging_buffer_memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            image_size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );

        let data = device
            .map_memory(
                staging_buffer_memory,
                0,
                image_size,
                MemoryMapFlags::empty(),
            )
            .unwrap();
        let pixels = image.into_raw();
        copy_nonoverlapping(pixels.as_ptr(), data as *mut u8, pixels.len());
        device.unmap_memory(staging_buffer_memory);

        let format = Format::R8G8B8A8_SRGB;
        let (texture_image, image_memory) = InternalVulkan::create_image(
            instance,
            physical_device,
            device,
            image_extent,
            format,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_preparing_the_texture_image
        InternalVulkan::transition_image_layout(
            device,
            graphics_queue,
            command_pool,
            texture_image,
            ImageLayout::UNDEFINED,
            ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        InternalVulkan::copy_buffer_to_image(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            texture_image,
            image_extent,
        );
        InternalVulkan::transition_image_layout(
            device,
            graphics_queue,
            command_pool,
            texture_image,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );

        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);

        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/01_Image_view_and_sampler.html#_texture_image_view
        let image_view = InternalVulkan::create_image_view(
            device,
            texture_image,
            format,
            ImageAspectFlags::COLOR,
        );

        (texture_image, image_memory, image_view)
    }

    unsafe fn create_texture_sampler(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
    ) -> Sampler {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/01_Image_view_and_sampler.html#_samplers
        let properties = instance.get_physical_device_properties(physical_device);

        let sampler_create_info = SamplerCreateInfo::default()
            .mag_filter(Filter::LINEAR)
            .min_filter(Filter::LINEAR)
            .address_mode_u(SamplerAddressMode::REPEAT)
            .address_mode_v(SamplerAddressMode::REPEAT)
            .address_mode_w(SamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(properties.limits.max_sampler_anisotropy)
            .border_color(BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(CompareOp::ALWAYS)
            .mipmap_mode(SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(0.0);

        device.create_sampler(&sampler_create_info, None).unwrap()
    }
}
