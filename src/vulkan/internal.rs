use ash::{
    vk::{
        AccessFlags, Buffer, BufferCopy, BufferCreateInfo, BufferImageCopy, BufferUsageFlags,
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPool, DependencyFlags, DeviceMemory, Extent2D, Extent3D,
        Fence, Format, FormatFeatureFlags, Image, ImageAspectFlags, ImageCreateInfo, ImageLayout,
        ImageMemoryBarrier, ImageSubresourceLayers, ImageSubresourceRange, ImageTiling, ImageType,
        ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, MemoryAllocateInfo,
        MemoryPropertyFlags, MemoryRequirements, Offset3D, PhysicalDevice, PipelineStageFlags,
        Queue, SampleCountFlags, SharingMode, SubmitInfo, QUEUE_FAMILY_IGNORED,
    },
    Device, Instance,
};

pub struct InternalVulkan {}

impl InternalVulkan {
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn create_image(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        extent: Extent2D,
        format: Format,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        memory_properties: MemoryPropertyFlags,
    ) -> (Image, DeviceMemory) {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_texture_image
        let image_info = ImageCreateInfo::default()
            .image_type(ImageType::TYPE_2D)
            .extent(Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .format(format)
            .tiling(tiling)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(usage)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .samples(SampleCountFlags::TYPE_1);
        let texture_image = device.create_image(&image_info, None).unwrap();

        let memory_requirements = device.get_image_memory_requirements(texture_image);
        let allocation_info = MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(InternalVulkan::find_memory_type(
                instance,
                physical_device,
                memory_requirements,
                memory_properties,
            ));
        let texture_image_memory = device.allocate_memory(&allocation_info, None).unwrap();

        device
            .bind_image_memory(texture_image, texture_image_memory, 0)
            .unwrap();

        (texture_image, texture_image_memory)
    }

    unsafe fn begin_single_time_commands(
        device: &Device,
        command_pool: CommandPool,
    ) -> CommandBuffer {
        let allocation_info = CommandBufferAllocateInfo::default()
            .level(CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);

        let command_buffer = device.allocate_command_buffers(&allocation_info).unwrap()[0];

        let begin_info =
            CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        device
            .begin_command_buffer(command_buffer, &begin_info)
            .unwrap();

        command_buffer
    }

    unsafe fn end_single_time_commands(
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
        command_buffer: CommandBuffer,
    ) {
        device.end_command_buffer(command_buffer).unwrap();

        let buffers = [command_buffer];
        let submit_info = SubmitInfo::default().command_buffers(&buffers);

        device
            .queue_submit(graphics_queue, &[submit_info], Fence::null())
            .unwrap();

        device.queue_wait_idle(graphics_queue).unwrap();
        device.free_command_buffers(command_pool, &[command_buffer])
    }

    pub unsafe fn transition_image_layout(
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
        image: Image,
        old_layout: ImageLayout,
        new_layout: ImageLayout,
    ) {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_layout_transitions
        let command_buffer = InternalVulkan::begin_single_time_commands(device, command_pool);

        let src_access_mask;
        let dst_access_mask;

        let src_stage_mask;
        let dst_stage_mask;

        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_transition_barrier_masks
        if old_layout == ImageLayout::UNDEFINED && new_layout == ImageLayout::TRANSFER_DST_OPTIMAL {
            src_access_mask = AccessFlags::empty();
            dst_access_mask = AccessFlags::TRANSFER_WRITE;

            src_stage_mask = PipelineStageFlags::TOP_OF_PIPE;
            dst_stage_mask = PipelineStageFlags::TRANSFER;
        } else if old_layout == ImageLayout::TRANSFER_DST_OPTIMAL
            && new_layout == ImageLayout::SHADER_READ_ONLY_OPTIMAL
        {
            src_access_mask = AccessFlags::TRANSFER_WRITE;
            dst_access_mask = AccessFlags::SHADER_READ;

            src_stage_mask = PipelineStageFlags::TRANSFER;
            dst_stage_mask = PipelineStageFlags::FRAGMENT_SHADER;
        } else {
            panic!("unsupported layout transition!");
        }

        let image_memory_barrier = ImageMemoryBarrier::default()
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(QUEUE_FAMILY_IGNORED)
            .image(image)
            .subresource_range(ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            })
            .src_access_mask(src_access_mask)
            .dst_access_mask(dst_access_mask);

        device.cmd_pipeline_barrier(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            DependencyFlags::empty(),
            &[],
            &[],
            &[image_memory_barrier],
        );

        InternalVulkan::end_single_time_commands(
            device,
            graphics_queue,
            command_pool,
            command_buffer,
        );
    }

    pub unsafe fn copy_buffer_to_image(
        device: &Device,
        command_pool: CommandPool,
        graphics_queue: Queue,
        buffer: Buffer,
        image: Image,
        extent: Extent2D,
    ) {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_copying_buffer_to_image
        let command_buffer = InternalVulkan::begin_single_time_commands(device, command_pool);

        let region = BufferImageCopy::default()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(ImageSubresourceLayers {
                aspect_mask: ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            })
            .image_offset(Offset3D { x: 0, y: 0, z: 0 })
            .image_extent(Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            });

        device.cmd_copy_buffer_to_image(
            command_buffer,
            buffer,
            image,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region],
        );

        InternalVulkan::end_single_time_commands(
            device,
            graphics_queue,
            command_pool,
            command_buffer,
        );
    }

    pub unsafe fn create_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        size: u64,
        usage_flags: BufferUsageFlags,
        memory_properties: MemoryPropertyFlags,
    ) -> (Buffer, DeviceMemory) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/02_Staging_buffer.html#_abstracting_buffer_creation
        let buffer_create_info = BufferCreateInfo::default()
            .size(size)
            .usage(usage_flags)
            .sharing_mode(SharingMode::EXCLUSIVE);
        let buffer = device.create_buffer(&buffer_create_info, None).unwrap();

        let memory_requirements = device.get_buffer_memory_requirements(buffer);

        let allocation_info = MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(InternalVulkan::find_memory_type(
                instance,
                physical_device,
                memory_requirements,
                memory_properties,
            ));
        let buffer_memory = device.allocate_memory(&allocation_info, None).unwrap();

        device.bind_buffer_memory(buffer, buffer_memory, 0).unwrap();

        (buffer, buffer_memory)
    }

    pub unsafe fn copy_buffer(
        device: &Device,
        command_pool: CommandPool,
        graphics_queue: Queue,
        src: Buffer,
        dst: Buffer,
        size: u64,
    ) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/02_Staging_buffer.html#_conclusion
        let command_buffer = InternalVulkan::begin_single_time_commands(device, command_pool);

        let copy_region = BufferCopy::default().src_offset(0).dst_offset(0).size(size);
        device.cmd_copy_buffer(command_buffer, src, dst, &[copy_region]);

        InternalVulkan::end_single_time_commands(
            device,
            graphics_queue,
            command_pool,
            command_buffer,
        );
    }

    unsafe fn find_memory_type(
        instance: &Instance,
        physical_device: PhysicalDevice,
        memory_requirements: MemoryRequirements,
        properties: MemoryPropertyFlags,
    ) -> u32 {
        let memory_properties = instance.get_physical_device_memory_properties(physical_device);

        (0..memory_properties.memory_type_count)
            .find(|index| {
                (memory_requirements.memory_type_bits & (1 << index)) != 0
                    && (memory_properties.memory_types[*index as usize].property_flags & properties)
                        == properties
            })
            .unwrap()
    }

    pub unsafe fn create_image_view(
        device: &Device,
        image: Image,
        format: Format,
        aspect_mask: ImageAspectFlags,
    ) -> ImageView {
        let image_view_create_info = ImageViewCreateInfo::default()
            .image(image)
            .view_type(ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(ImageSubresourceRange {
                aspect_mask,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        device
            .create_image_view(&image_view_create_info, None)
            .unwrap()
    }

    fn find_supported_format(
        instance: &Instance,
        physical_device: PhysicalDevice,
        formats: Vec<Format>,
        tiling: ImageTiling,
        features: FormatFeatureFlags,
    ) -> Format {
        for format in formats {
            let properties =
                unsafe { instance.get_physical_device_format_properties(physical_device, format) };

            if tiling == ImageTiling::LINEAR
                && (properties.linear_tiling_features & features) == features
            {
                return format;
            }
            if tiling == ImageTiling::OPTIMAL
                && (properties.optimal_tiling_features & features) == features
            {
                return format;
            }
        }

        panic!("unable to find supported format!");
    }

    pub fn find_depth_format(instance: &Instance, physical_device: PhysicalDevice) -> Format {
        InternalVulkan::find_supported_format(
            instance,
            physical_device,
            vec![
                Format::D32_SFLOAT,
                Format::D32_SFLOAT_S8_UINT,
                Format::D24_UNORM_S8_UINT,
            ],
            ImageTiling::OPTIMAL,
            FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        )
    }
}
