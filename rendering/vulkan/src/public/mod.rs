mod setup;

use super::{
    internal::Internal,
    structs::{ImageData, StorageBufferObject, Vertex, MVP},
    Vulkan,
};

use ash::{
    vk::{
        BorderColor, Buffer, BufferUsageFlags, ClearColorValue, ClearDepthStencilValue, ClearValue,
        CommandBuffer, CommandBufferBeginInfo, CommandPool, CompareOp, DescriptorBufferInfo,
        DescriptorImageInfo, DescriptorPool, DescriptorPoolCreateInfo, DescriptorPoolSize,
        DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorSetLayoutBinding,
        DescriptorSetLayoutCreateInfo, DescriptorType, DeviceMemory, Extent2D, Filter, Format,
        Framebuffer, ImageAspectFlags, ImageLayout, ImageTiling, ImageUsageFlags, ImageView,
        IndexType, MemoryMapFlags, MemoryPropertyFlags, Offset2D, PhysicalDevice, Pipeline,
        PipelineBindPoint, PipelineLayout, Queue, Rect2D, RenderPass, RenderPassBeginInfo, Sampler,
        SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode, ShaderStageFlags,
        SubpassContents, Viewport, WriteDescriptorSet,
    },
    Device, Instance,
};

use std::{
    mem::{size_of, size_of_val},
    ptr::copy_nonoverlapping,
    rc::Rc,
};

impl Vulkan {
    #[must_use]
    pub fn create_depth(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: Rc<Device>,
        extent: Extent2D,
    ) -> ImageData {
        let depth_format = Internal::find_depth_format(instance, physical_device);
        let (depth_image, depth_image_memory) = Internal::create_image(
            instance,
            physical_device,
            &device,
            extent,
            depth_format,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );
        let depth_image_view = Internal::create_image_view(
            &device,
            depth_image,
            depth_format,
            ImageAspectFlags::DEPTH,
        );

        ImageData::create(depth_image, depth_image_memory, depth_image_view, device)
    }

    pub fn begin_render_pass(
        device: &Device,
        framebuffers: &[Framebuffer],
        image_index: usize,
        command_buffer: CommandBuffer,
        extent: Extent2D,
        render_pass: RenderPass,
        pipeline: Pipeline,
    ) {
        let begin_info = CommandBufferBeginInfo::default();
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

        let viewports = [Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(extent.width as f32)
            .height(extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)];
        let scissors = [Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent,
        }];

        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }
            .expect("Failed to begin command buffer!");
        unsafe {
            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                SubpassContents::INLINE,
            );

            device.cmd_set_viewport(command_buffer, 0, &viewports);
            device.cmd_set_scissor(command_buffer, 0, &scissors);

            device.cmd_bind_pipeline(command_buffer, PipelineBindPoint::GRAPHICS, pipeline);
        };
    }

    pub fn bind_buffers(
        device: &Device,
        command_buffer: CommandBuffer,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
    ) {
        unsafe {
            device.cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer], &[0]);
            device.cmd_bind_index_buffer(command_buffer, index_buffer, 0, IndexType::UINT16);
        }
    }

    pub fn draw_indexed_instanced(
        device: &Device,
        command_buffer: CommandBuffer,
        pipeline_layout: PipelineLayout,
        descriptor_set: DescriptorSet,
        index_count: u32,
        mvps: &[MVP],
        mvp_buffer: &StorageBufferObject,
    ) {
        mvp_buffer.update_data(mvps);
        unsafe {
            device.cmd_bind_descriptor_sets(
                command_buffer,
                PipelineBindPoint::GRAPHICS,
                pipeline_layout,
                0,
                &[descriptor_set],
                &[],
            );

            device.cmd_draw_indexed(command_buffer, index_count, mvps.len() as u32, 0, 0, 0);
        }
    }

    pub fn end_render_pass(device: &Device, command_buffer: CommandBuffer) {
        unsafe {
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer)
        }
        .expect("Failed to end command buffer!");
    }

    #[must_use]
    pub fn create_vertex_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        vertices: &[Vertex],
        command_pool: CommandPool,
        graphics_queue: Queue,
    ) -> (Buffer, DeviceMemory) {
        let buffer_size = size_of_val(vertices) as u64;

        let (staging_buffer, staging_buffer_memory) = Internal::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );

        let data = unsafe {
            device.map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                MemoryMapFlags::empty(),
            )
        }
        .expect("Failed to map memory for staging vertex buffer!")
        .cast::<Vertex>();
        unsafe {
            copy_nonoverlapping(vertices.as_ptr(), data, vertices.len());
            device.unmap_memory(staging_buffer_memory);
        };

        let (vertex_buffer, vertex_buffer_memory) = Internal::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::VERTEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        Internal::copy_buffer(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            vertex_buffer,
            buffer_size,
        );

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        };

        (vertex_buffer, vertex_buffer_memory)
    }

    #[must_use]
    pub fn create_index_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        indices: &[u32],
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Buffer, DeviceMemory) {
        let buffer_size = size_of_val(indices) as u64;

        let (staging_buffer, staging_buffer_memory) = Internal::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );

        let data = unsafe {
            device.map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                MemoryMapFlags::empty(),
            )
        }
        .expect("Failed to map memory for staging index buffer")
        .cast::<u32>();
        unsafe {
            copy_nonoverlapping(indices.as_ptr(), data, indices.len());
            device.unmap_memory(staging_buffer_memory);
        };

        let (index_buffer, index_buffer_memory) = Internal::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::INDEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        Internal::copy_buffer(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            index_buffer,
            buffer_size,
        );

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        };

        (index_buffer, index_buffer_memory)
    }

    #[must_use]
    pub fn create_descriptors(
        device: &Device,
        in_flight: u32,
        texture_count: u32,
    ) -> (DescriptorSetLayout, DescriptorPool) {
        // layout
        let mvp_binding = DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::STORAGE_BUFFER)
            .stage_flags(ShaderStageFlags::VERTEX);
        let texture_binding = DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_count(texture_count)
            .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
            .stage_flags(ShaderStageFlags::FRAGMENT);

        let bindings = [mvp_binding, texture_binding];
        let layout_create_info = DescriptorSetLayoutCreateInfo::default().bindings(&bindings);

        let descriptor_set_layout =
            unsafe { device.create_descriptor_set_layout(&layout_create_info, None) }
                .expect("Failed to create descriptor set layout!");

        // pool
        let pool_sizes = [
            DescriptorPoolSize::default()
                .descriptor_count(in_flight)
                .ty(DescriptorType::STORAGE_BUFFER),
            DescriptorPoolSize::default()
                .descriptor_count(texture_count * in_flight)
                .ty(DescriptorType::COMBINED_IMAGE_SAMPLER),
        ];
        let pool_info = DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(in_flight);

        let descriptor_pool = unsafe { device.create_descriptor_pool(&pool_info, None) }
            .expect("Failed to create descriptor pool!");

        (descriptor_set_layout, descriptor_pool)
    }

    #[must_use]
    pub fn create_descriptor_sets(
        device: &Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_layout: DescriptorSetLayout,
        amount: usize,
    ) -> Vec<DescriptorSet> {
        let descriptor_set_layouts = &vec![descriptor_set_layout; amount];
        let descriptor_allocation_info = DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(descriptor_set_layouts);
        unsafe { device.allocate_descriptor_sets(&descriptor_allocation_info) }
            .expect("Failed to allocate descriptor sets!")
    }

    pub fn update_texture_descriptors(
        device: &Device,
        descriptor_set: DescriptorSet,
        texture_image_views: &[ImageView],
        texture_sampler: Sampler,
    ) {
        let mut image_infos: Vec<DescriptorImageInfo> =
            Vec::with_capacity(texture_image_views.len());

        for image_view in texture_image_views {
            image_infos.push(
                DescriptorImageInfo::default()
                    .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .image_view(*image_view)
                    .sampler(texture_sampler),
            );
        }

        let descriptor_writes = [WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(1)
            .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&image_infos)];
        unsafe {
            device.update_descriptor_sets(&descriptor_writes, &[]);
        };
    }

    pub fn update_mvp_descriptors(
        device: &Device,
        descriptor_set: DescriptorSet,
        entity_count: usize,
        mvp_buffer: Buffer,
    ) {
        let mvp_buffer_info = [DescriptorBufferInfo::default()
            .buffer(mvp_buffer)
            .offset(0)
            .range((size_of::<MVP>() * entity_count) as u64)];
        let descriptor_writes = [WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(0)
            .descriptor_type(DescriptorType::STORAGE_BUFFER)
            .buffer_info(&mvp_buffer_info)];
        unsafe {
            device.update_descriptor_sets(&descriptor_writes, &[]);
        };
    }

    #[must_use]
    pub fn create_image_data(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: Rc<Device>,
        graphics_queue: Queue,
        command_pool: CommandPool,
        image_extent: Extent2D,
        image_data: &[u8],
    ) -> ImageData {
        let image_size = u64::from(image_extent.width * image_extent.height * 4);

        let (staging_buffer, staging_buffer_memory) = Internal::create_buffer(
            instance,
            physical_device,
            &device,
            image_size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );

        let data = unsafe {
            device.map_memory(
                staging_buffer_memory,
                0,
                image_size,
                MemoryMapFlags::empty(),
            )
        }
        .expect("Failed to map memory for staging buffer while creating image data!")
        .cast::<u8>();

        unsafe {
            copy_nonoverlapping(image_data.as_ptr(), data, image_data.len());
            device.unmap_memory(staging_buffer_memory);
        };

        let format = Format::R8G8B8A8_SRGB;
        let (image, image_memory) = Internal::create_image(
            instance,
            physical_device,
            &device,
            image_extent,
            format,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        Internal::transition_image_layout(
            &device,
            graphics_queue,
            command_pool,
            image,
            ImageLayout::UNDEFINED,
            ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        Internal::copy_buffer_to_image(
            &device,
            command_pool,
            graphics_queue,
            staging_buffer,
            image,
            image_extent,
        );
        Internal::transition_image_layout(
            &device,
            graphics_queue,
            command_pool,
            image,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        };

        let image_view =
            Internal::create_image_view(&device, image, format, ImageAspectFlags::COLOR);

        ImageData::create(image, image_memory, image_view, device)
    }

    #[must_use]
    pub fn create_texture_sampler(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
    ) -> Sampler {
        let properties = unsafe { instance.get_physical_device_properties(physical_device) };

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

        unsafe { device.create_sampler(&sampler_create_info, None) }
            .expect("Failed to create sampler for texture!")
    }
}
