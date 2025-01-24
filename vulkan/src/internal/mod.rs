use ash::{
    khr::{surface, swapchain},
    util::read_spv,
    vk::{
        AccessFlags, Buffer, BufferCopy, BufferCreateInfo, BufferImageCopy, BufferUsageFlags,
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPool, DependencyFlags, DescriptorSetLayout, DeviceMemory,
        ExtensionProperties, Extent2D, Extent3D, Fence, Format, FormatFeatureFlags, Image,
        ImageAspectFlags, ImageCreateInfo, ImageLayout, ImageMemoryBarrier, ImageSubresourceLayers,
        ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView,
        ImageViewCreateInfo, ImageViewType, MemoryAllocateInfo, MemoryMapFlags,
        MemoryPropertyFlags, MemoryRequirements, Offset3D, PhysicalDevice, PhysicalDeviceFeatures,
        PhysicalDeviceFeatures2, PhysicalDeviceVulkan12Features, PipelineLayout,
        PipelineLayoutCreateInfo, PipelineShaderStageCreateInfo, PipelineStageFlags,
        PresentModeKHR, Queue, QueueFlags, SampleCountFlags, ShaderModuleCreateInfo,
        ShaderStageFlags, SharingMode, SubmitInfo, SurfaceKHR, QUEUE_FAMILY_IGNORED, TRUE,
    },
    Device, Instance,
};
use std::{
    ffi::CStr,
    fs::File,
    io::{Cursor, Read, Result},
    mem::size_of,
};

use super::structs::{ShaderModules, MVP};

pub struct Internal;

impl Internal {
    #[allow(clippy::too_many_arguments)]
    pub fn create_image(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        extent: Extent2D,
        format: Format,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        memory_properties: MemoryPropertyFlags,
    ) -> (Image, DeviceMemory) {
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

        let image =
            unsafe { device.create_image(&image_info, None) }.expect("Failed to create image!");
        let memory_requirements = unsafe { device.get_image_memory_requirements(image) };

        let allocation_info = MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(Internal::find_memory_type(
                instance,
                physical_device,
                memory_requirements,
                memory_properties,
            ));

        let image_memory = unsafe { device.allocate_memory(&allocation_info, None) }
            .expect("Failed to allocate memory for image!");
        unsafe { device.bind_image_memory(image, image_memory, 0) }
            .expect("Failed to bind image memory!");

        (image, image_memory)
    }

    fn begin_single_time_commands(device: &Device, command_pool: CommandPool) -> CommandBuffer {
        let allocation_info = CommandBufferAllocateInfo::default()
            .level(CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);

        let command_buffer = unsafe { device.allocate_command_buffers(&allocation_info) }
            .expect("Failed to allocate command buffer for single time command!")[0];

        let begin_info =
            CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }
            .expect("Failed to begin command buffer for single time command!");

        command_buffer
    }

    fn end_single_time_commands(
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
        command_buffer: CommandBuffer,
    ) {
        unsafe { device.end_command_buffer(command_buffer) }
            .expect("Failed to end command buffer for single time command!");

        let buffers = [command_buffer];
        let submit_info = SubmitInfo::default().command_buffers(&buffers);

        unsafe { device.queue_submit(graphics_queue, &[submit_info], Fence::null()) }
            .expect("Failed to submit single time command queue!");

        unsafe { device.queue_wait_idle(graphics_queue) }
            .expect("Failed to wait for queue idle after single time command!");
        unsafe {
            device.free_command_buffers(command_pool, &[command_buffer]);
        };
    }

    pub fn transition_image_layout(
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
        image: Image,
        old_layout: ImageLayout,
        new_layout: ImageLayout,
    ) {
        let command_buffer = Internal::begin_single_time_commands(device, command_pool);

        let src_access_mask;
        let dst_access_mask;

        let src_stage_mask;
        let dst_stage_mask;

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

        unsafe {
            device.cmd_pipeline_barrier(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        Internal::end_single_time_commands(device, graphics_queue, command_pool, command_buffer);
    }

    pub fn copy_buffer_to_image(
        device: &Device,
        command_pool: CommandPool,
        graphics_queue: Queue,
        buffer: Buffer,
        image: Image,
        extent: Extent2D,
    ) {
        let command_buffer = Internal::begin_single_time_commands(device, command_pool);

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

        unsafe {
            device.cmd_copy_buffer_to_image(
                command_buffer,
                buffer,
                image,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &[region],
            );
        };

        Internal::end_single_time_commands(device, graphics_queue, command_pool, command_buffer);
    }

    pub fn create_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        size: u64,
        usage_flags: BufferUsageFlags,
        memory_properties: MemoryPropertyFlags,
    ) -> (Buffer, DeviceMemory) {
        let buffer_create_info = BufferCreateInfo::default()
            .size(size)
            .usage(usage_flags)
            .sharing_mode(SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&buffer_create_info, None) }
            .expect("Failed to create buffer!");
        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        let allocation_info = MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(Internal::find_memory_type(
                instance,
                physical_device,
                memory_requirements,
                memory_properties,
            ));

        let buffer_memory = unsafe { device.allocate_memory(&allocation_info, None) }
            .expect("Failed to allocate buffer memory!");
        unsafe { device.bind_buffer_memory(buffer, buffer_memory, 0) }
            .expect("Failed to bind buffer memory!");

        (buffer, buffer_memory)
    }

    pub fn copy_buffer(
        device: &Device,
        command_pool: CommandPool,
        graphics_queue: Queue,
        src: Buffer,
        dst: Buffer,
        size: u64,
    ) {
        let command_buffer = Internal::begin_single_time_commands(device, command_pool);
        let copy_region = BufferCopy::default().src_offset(0).dst_offset(0).size(size);

        unsafe {
            device.cmd_copy_buffer(command_buffer, src, dst, &[copy_region]);
        };

        Internal::end_single_time_commands(device, graphics_queue, command_pool, command_buffer);
    }

    fn find_memory_type(
        instance: &Instance,
        physical_device: PhysicalDevice,
        memory_requirements: MemoryRequirements,
        properties: MemoryPropertyFlags,
    ) -> u32 {
        let memory_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };

        (0..memory_properties.memory_type_count)
            .find(|index| {
                (memory_requirements.memory_type_bits & (1 << index)) != 0
                    && (memory_properties.memory_types[*index as usize].property_flags & properties)
                        == properties
            })
            .expect("Failed to find fitting memory type!")
    }

    pub fn create_image_view(
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

        unsafe { device.create_image_view(&image_view_create_info, None) }
            .expect("Failed to create image view!")
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
        Internal::find_supported_format(
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

    pub fn validate_physical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> Option<(PhysicalDevice, u32)> {
        let mut features = PhysicalDeviceFeatures2::default();
        let mut device_features_12 = PhysicalDeviceVulkan12Features::default();
        features = features.push_next(&mut device_features_12);

        unsafe {
            instance.get_physical_device_features2(physical_device, &mut features);
        };

        if !features.features.sampler_anisotropy == TRUE
            && device_features_12.runtime_descriptor_array == TRUE
            && device_features_12.shader_sampled_image_array_non_uniform_indexing == TRUE
        {
            return None;
        }

        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let queue_family_index =
            queue_family_properties
                .iter()
                .enumerate()
                .find_map(|(index, info)| {
                    if !info.queue_flags.contains(QueueFlags::GRAPHICS) {
                        return None;
                    }

                    let surface_supported = unsafe {
                        surface_loader.get_physical_device_surface_support(
                            physical_device,
                            index as u32,
                            surface,
                        )
                    }
                    .unwrap_or(false);

                    if !surface_supported {
                        return None;
                    }

                    Some(index as u32)
                });

        let index = queue_family_index?;

        Some((physical_device, index))
    }

    pub fn create_shader_modules(
        device: &Device,
        vertex_path: &str,
        fragment_path: &str,
    ) -> ShaderModules {
        let vertex_shader_code = read_spv(&mut Cursor::new(
            &read_bytes_from_file(vertex_path)
                .expect(&("Could not read file: ".to_string() + vertex_path)),
        ))
        .expect("Failed to read vertex shader spv!");
        let vertex_shader_create_info = ShaderModuleCreateInfo::default().code(&vertex_shader_code);
        let vertex_module =
            unsafe { device.create_shader_module(&vertex_shader_create_info, None) }
                .expect("Failed to create vertex shader module!");

        let fragment_shader_code = read_spv(&mut Cursor::new(
            &read_bytes_from_file(fragment_path)
                .expect(&("Could not read file: ".to_string() + fragment_path)),
        ))
        .expect("Failed to read fragment shader spv!");
        let fragment_shader_create_info =
            ShaderModuleCreateInfo::default().code(&fragment_shader_code);
        let fragment_module =
            unsafe { device.create_shader_module(&fragment_shader_create_info, None) }
                .expect("Failed to create fragment shader module!");

        ShaderModules {
            vertex_module,
            fragment_module,
        }
    }

    pub fn create_pipeline_layout(
        device: &Device,
        descriptor_set_layouts: &[DescriptorSetLayout],
    ) -> PipelineLayout {
        let pipeline_layout_info =
            PipelineLayoutCreateInfo::default().set_layouts(descriptor_set_layouts);

        unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) }
            .expect("Failed to create pipeline layout!")
    }

    pub fn create_shader_stages(
        shader_modules: &ShaderModules,
    ) -> Vec<PipelineShaderStageCreateInfo> {
        let stage_entry_point = CStr::from_bytes_with_nul("main\0".as_bytes())
            .expect("Failed to convert bytes to cstr!");
        vec![
            PipelineShaderStageCreateInfo::default()
                .stage(ShaderStageFlags::VERTEX)
                .module(shader_modules.vertex_module)
                .name(stage_entry_point),
            PipelineShaderStageCreateInfo::default()
                .stage(ShaderStageFlags::FRAGMENT)
                .module(shader_modules.fragment_module)
                .name(stage_entry_point),
        ]
    }

    pub fn get_swapchain_present_mode(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> PresentModeKHR {
        let Some(mailbox) = unsafe {
            surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
        }
        .expect("Failed to get surface present modes!")
        .iter()
        .copied()
        .find(|&mode| mode == PresentModeKHR::MAILBOX) else {
            return PresentModeKHR::FIFO;
        };

        mailbox
    }

    pub fn create_ssbo(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        capacity: usize,
    ) -> (Buffer, DeviceMemory, *mut MVP) {
        let buffer_size = (capacity * size_of::<MVP>()) as u64;

        let (buffer, memory) = Internal::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::STORAGE_BUFFER,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );
        let mapped = unsafe { device.map_memory(memory, 0, buffer_size, MemoryMapFlags::empty()) }
            .expect("Failed to map memory for SSBO!")
            .cast::<MVP>();

        (buffer, memory, mapped)
    }

    pub fn is_extension_supported(extensions: &[ExtensionProperties], required: &CStr) -> bool {
        extensions.iter().any(|ext| {
            let ext_name = &ext.extension_name;
            let len = ext_name
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(ext_name.len());
            let ext_name_u8 = &ext_name[..len].iter().map(|&c| c as u8).collect::<Vec<_>>();
            ext_name_u8 == required.to_bytes()
        })
    }

    pub fn get_device_extensions(
        instance: &Instance,
        physical_device: PhysicalDevice,
    ) -> [*const i8; 1] {
        let supported_extensions =
            unsafe { instance.enumerate_device_extension_properties(physical_device) }
                .expect("Failed to enumerate device extension properties!");
        assert!(
            Internal::is_extension_supported(&supported_extensions, swapchain::NAME),
            "Required extension {:?} is not supported!",
            swapchain::NAME
        );

        [swapchain::NAME.as_ptr()]
    }

    pub fn get_12_features(
        instance: &Instance,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceVulkan12Features<'static> {
        let mut supported_vulkan12_features = PhysicalDeviceVulkan12Features::default();
        let mut features2 =
            PhysicalDeviceFeatures2::default().push_next(&mut supported_vulkan12_features);

        unsafe {
            instance.get_physical_device_features2(physical_device, &mut features2);
        }

        assert!(
            supported_vulkan12_features.runtime_descriptor_array != 0,
            "Physical device does not support runtime descriptor array!"
        );
        assert!(
            supported_vulkan12_features.shader_sampled_image_array_non_uniform_indexing != 0,
            "Physical device does not support shader sampled image array non-uniform indexing!"
        );

        PhysicalDeviceVulkan12Features::default()
            .runtime_descriptor_array(true)
            .shader_sampled_image_array_non_uniform_indexing(true)
    }

    pub fn get_13_features(
        instance: &Instance,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        let supported_features = unsafe { instance.get_physical_device_features(physical_device) };
        assert!(
            supported_features.sampler_anisotropy != 0,
            "Physical device does not support sampler anisotropy!"
        );

        PhysicalDeviceFeatures::default().sampler_anisotropy(true)
    }
}

fn read_bytes_from_file(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
