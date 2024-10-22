use crate::{
    constants::TITLE,
    ecs::system::RenderSystem,
    structs::{ImageData, ModelViewProjection, StorageBufferObject, Vertex},
    vulkan::internal,
};
use ash::{
    khr::{surface, swapchain},
    vk::{
        AccessFlags, ApplicationInfo, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, BorderColor, Buffer, BufferUsageFlags,
        ClearColorValue, ClearDepthStencilValue, ClearValue, ColorComponentFlags, CommandBuffer,
        CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel, CommandPool,
        CommandPoolCreateFlags, CommandPoolCreateInfo, CompareOp, CompositeAlphaFlagsKHR,
        CullModeFlags, DescriptorBufferInfo, DescriptorImageInfo, DescriptorPool,
        DescriptorPoolCreateInfo, DescriptorPoolSize, DescriptorSet, DescriptorSetAllocateInfo,
        DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo,
        DescriptorType, DeviceCreateInfo, DeviceMemory, DeviceQueueCreateInfo, DynamicState,
        Extent2D, Fence, FenceCreateFlags, FenceCreateInfo, Filter, Format, Framebuffer,
        FramebufferCreateInfo, FrontFace, GraphicsPipelineCreateInfo, Image, ImageAspectFlags,
        ImageLayout, ImageTiling, ImageUsageFlags, ImageView, IndexType, InstanceCreateInfo,
        LogicOp, MemoryMapFlags, MemoryPropertyFlags, Offset2D, PhysicalDevice,
        PhysicalDeviceFeatures, PhysicalDeviceVulkan12Features, Pipeline, PipelineBindPoint,
        PipelineCache, PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
        PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineMultisampleStateCreateInfo,
        PipelineRasterizationStateCreateInfo, PipelineStageFlags,
        PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode,
        PresentModeKHR, PrimitiveTopology, Queue, Rect2D, RenderPass, RenderPassBeginInfo,
        RenderPassCreateInfo, SampleCountFlags, Sampler, SamplerAddressMode, SamplerCreateInfo,
        SamplerMipmapMode, Semaphore, SemaphoreCreateInfo, ShaderStageFlags, SharingMode,
        SubpassContents, SubpassDependency, SubpassDescription, SurfaceKHR, SwapchainCreateInfoKHR,
        SwapchainKHR, Viewport, WriteDescriptorSet, API_VERSION_1_3, SUBPASS_EXTERNAL,
    },
    Device, Entry, Instance,
};
use ash_window::{create_surface, enumerate_required_extensions};
use image::{ImageBuffer, Rgba};
use std::{
    array::from_ref,
    ffi::CStr,
    mem::{size_of, size_of_val},
    ptr::copy_nonoverlapping,
};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct VulkanWrapper;

impl VulkanWrapper {
    pub fn create_vulkan_instance(entry: &Entry, window: &winit::window::Window) -> Instance {
        let display_handle = window
            .display_handle()
            .expect("Failed to get display handle!")
            .as_raw();
        let extension_names = enumerate_required_extensions(display_handle)
            .expect("Failed to get required extensions!")
            .to_vec();

        let application_name = TITLE.to_string() + "\0";
        let application_name_cstr = CStr::from_bytes_with_nul(application_name.as_bytes())
            .expect("Failed to convert bytes to cstr!");
        let application_info = ApplicationInfo::default()
            .application_name(application_name_cstr)
            .application_version(1)
            .api_version(API_VERSION_1_3);

        let create_info = InstanceCreateInfo::default()
            .enabled_extension_names(&extension_names)
            .application_info(&application_info);

        unsafe { entry.create_instance(&create_info, None) }
            .expect("Failed to create Vulkan Instance!")
    }

    pub fn create_surface(
        window: &winit::window::Window,
        entry: &Entry,
        instance: &Instance,
    ) -> (SurfaceKHR, surface::Instance) {
        let display_handle = window
            .display_handle()
            .expect("Failed to get display handle!")
            .as_raw();
        let window_handle = window
            .window_handle()
            .expect("Failed to get window handle!")
            .as_raw();

        let surface =
            unsafe { create_surface(entry, instance, display_handle, window_handle, None) }
                .expect("Failed to create a surface!");
        let surface_loader = surface::Instance::new(entry, instance);

        (surface, surface_loader)
    }

    pub fn find_physical_device(
        instance: &Instance,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, u32) {
        let devices = unsafe { instance.enumerate_physical_devices() }
            .expect("Failed to enumerate physical devices!");
        devices
            .iter()
            .find_map(|device| {
                if !internal::InternalVulkan::device_features_available(instance, *device) {
                    return None;
                }

                internal::InternalVulkan::get_queue_family_index(
                    instance,
                    surface,
                    surface_loader,
                    *device,
                )
                .map(|queue_index| (*device, queue_index))
            })
            .expect("Failed to find physical device that is valid!")
    }

    pub fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Device {
        let queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0]);

        let device_features = PhysicalDeviceFeatures::default().sampler_anisotropy(true);
        let mut vulkan12_features = PhysicalDeviceVulkan12Features::default()
            .runtime_descriptor_array(true)
            .shader_sampled_image_array_non_uniform_indexing(true);
        let device_extensions = [swapchain::NAME.as_ptr()];

        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(from_ref(&queue_create_info))
            .enabled_extension_names(&device_extensions)
            .enabled_features(&device_features)
            .push_next(&mut vulkan12_features);

        unsafe { instance.create_device(physical_device, &device_create_info, None) }
            .expect("Failed to create logical device!")
    }

    pub fn create_swapchain(
        instance: &Instance,
        surface: SurfaceKHR,
        device: &Device,
        physical_device: PhysicalDevice,
        surface_loader: &surface::Instance,
    ) -> (SwapchainKHR, swapchain::Device, Format, Extent2D) {
        let swapchain_loader = swapchain::Device::new(instance, device);
        let surface_capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
        }
        .expect("Failed to get surface capabilities!");
        let surface_format =
            unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface) }
                .expect("Failed to get surface formats!")[0]; /* in most cases it’s okay to just settle with the first format that is specified */
        let mut min_image_count = surface_capabilities.min_image_count + 1;
        if min_image_count > surface_capabilities.max_image_count {
            min_image_count = surface_capabilities.max_image_count;
        }

        let present_mode = unsafe {
            surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
        }
        .expect("Failed to get surface present modes!")
        .iter()
        .copied()
        .find(|&mode| mode == PresentModeKHR::MAILBOX)
        .unwrap_or(PresentModeKHR::FIFO);

        let extent = surface_capabilities.current_extent;
        let format = surface_format.format;
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(min_image_count)
            .image_format(format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);
        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None) }
            .expect("Failed to create swapchain!");

        (swapchain, swapchain_loader, format, extent)
    }

    pub fn create_image_views(
        swapchain_loader: &swapchain::Device,
        swapchain: SwapchainKHR,
        format: Format,
        device: &Device,
    ) -> (Vec<Image>, Vec<ImageView>) {
        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }
            .expect("Failed to get swapchain images!");
        let mut image_views: Vec<ImageView> = Vec::with_capacity(images.len());

        for image in &images {
            image_views.push(internal::InternalVulkan::create_image_view(
                device,
                *image,
                format,
                ImageAspectFlags::COLOR,
            ));
        }
        (images, image_views)
    }

    pub fn create_render_pass(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        format: Format,
    ) -> RenderPass {
        let color_attachment = AttachmentDescription::default()
            .format(format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR);
        let depth_attachment = AttachmentDescription::default()
            .format(internal::InternalVulkan::find_depth_format(
                instance,
                physical_device,
            ))
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let attachments = [color_attachment, depth_attachment];

        let color_attachments = [AttachmentReference::default()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let depth_stencil_attachment = AttachmentReference::default()
            .attachment(1)
            .layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let subpasses = [SubpassDescription::default()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments)
            .depth_stencil_attachment(&depth_stencil_attachment)];

        let dependencies = [SubpassDependency::default()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(
                PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | PipelineStageFlags::LATE_FRAGMENT_TESTS,
            )
            .src_access_mask(AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
            .dst_stage_mask(
                PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            )
            .dst_access_mask(
                AccessFlags::COLOR_ATTACHMENT_WRITE | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            )];

        let render_pass_info = RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        unsafe { device.create_render_pass(&render_pass_info, None) }
            .expect("Failed to create render pass!")
    }

    pub fn create_graphics_pipeline(
        device: &Device,
        extent: Extent2D,
        render_pass: RenderPass,
        descriptor_set_layouts: &[DescriptorSetLayout],
    ) -> (PipelineLayout, Pipeline) {
        let pipeline_layout =
            internal::InternalVulkan::create_pipeline_layout(device, descriptor_set_layouts);
        let (shader_stages, shader_modules) =
            internal::InternalVulkan::create_shader_stages(device);
        let binding_descriptions = [Vertex::get_binding_description()];
        let attribute_descriptions = Vertex::get_attribute_descriptions();
        let vertex_input_state = PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions);
        let dynamic_state = PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&[DynamicState::SCISSOR, DynamicState::VIEWPORT]);
        let input_assembly = PipelineInputAssemblyStateCreateInfo::default()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);
        let viewports = [Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(extent.width as f32)
            .height(extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)];
        let scissors = [Rect2D::default()
            .offset(Offset2D { x: 0, y: 0 })
            .extent(extent)];
        let viewport_state = PipelineViewportStateCreateInfo::default()
            .scissor_count(1)
            .scissors(&scissors)
            .viewport_count(1)
            .viewports(&viewports);
        let rasterizer = PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0);
        let multisampling = PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0)
            .sample_mask(&[])
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);
        let depth_stencil_state = PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(false);
        let color_blend_attachments = [PipelineColorBlendAttachmentState::default()
            .color_write_mask(ColorComponentFlags::RGBA)
            .blend_enable(true)
            .src_color_blend_factor(BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ONE)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)];
        let color_blending = PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(LogicOp::COPY)
            .attachments(&color_blend_attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let pipeline_infos = [GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .depth_stencil_state(&depth_stencil_state)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)];
        let graphics_pipeline = unsafe {
            device.create_graphics_pipelines(PipelineCache::null(), &pipeline_infos, None)
        }
        .expect("Failed to create graphics pipelines!")[0];

        unsafe {
            device.destroy_shader_module(shader_modules.vertex_module, None);
            device.destroy_shader_module(shader_modules.fragment_module, None);
        };

        (pipeline_layout, graphics_pipeline)
    }

    pub fn create_framebuffers(
        device: &Device,
        render_pass: RenderPass,
        image_views: &[ImageView],
        depth_image_view: ImageView,
        extent: Extent2D,
    ) -> Vec<Framebuffer> {
        let mut framebuffers: Vec<Framebuffer> = Vec::with_capacity(image_views.len());

        for image_view in image_views {
            let attachments = [*image_view, depth_image_view];
            let framebuffer_create_info = FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);
            let framebuffer = unsafe { device.create_framebuffer(&framebuffer_create_info, None) }
                .expect("Failed to create framebuffer!");
            framebuffers.push(framebuffer);
        }

        framebuffers
    }

    pub fn create_command_buffers(
        device: &Device,
        queue_family_index: u32,
        amount: usize,
    ) -> (Vec<CommandPool>, Vec<CommandBuffer>) {
        let mut command_pools: Vec<CommandPool> = Vec::with_capacity(amount);
        let mut command_buffers: Vec<CommandBuffer> = Vec::with_capacity(amount);

        let pool_create_info = CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(CommandPoolCreateFlags::empty());
        for _ in 0..amount {
            let command_pool = unsafe { device.create_command_pool(&pool_create_info, None) }
                .expect("Failed to create command pool!");
            command_pools.push(command_pool);

            let allocation_info = CommandBufferAllocateInfo::default()
                .command_pool(command_pool)
                .level(CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);
            let command_buffer = unsafe { device.allocate_command_buffers(&allocation_info) }
                .expect("Failed to create command buffers!")[0];
            command_buffers.push(command_buffer);
        }

        (command_pools, command_buffers)
    }

    pub fn create_sync(
        device: &Device,
        amount: usize,
    ) -> (Vec<Semaphore>, Vec<Semaphore>, Vec<Fence>) {
        let mut image_available_semaphores: Vec<Semaphore> = Vec::with_capacity(amount);
        let mut render_finished_semaphores: Vec<Semaphore> = Vec::with_capacity(amount);
        let mut in_flight_fence: Vec<Fence> = Vec::with_capacity(amount);

        let semaphore_create_info = SemaphoreCreateInfo::default();
        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

        for _ in 0..amount {
            image_available_semaphores.push(
                unsafe { device.create_semaphore(&semaphore_create_info, None) }
                    .expect("Failed to create semaphore!"),
            );
            render_finished_semaphores.push(
                unsafe { device.create_semaphore(&semaphore_create_info, None) }
                    .expect("Failed to create semaphore!"),
            );
            in_flight_fence.push(
                unsafe { device.create_fence(&fence_create_info, None) }
                    .expect("Failed to create fence!"),
            );
        }

        (
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fence,
        )
    }

    pub fn create_depth(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        extent: Extent2D,
    ) -> ImageData {
        let depth_format = internal::InternalVulkan::find_depth_format(instance, physical_device);
        let (depth_image, depth_image_memory) = internal::InternalVulkan::create_image(
            instance,
            physical_device,
            device,
            extent,
            depth_format,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );
        let depth_image_view = internal::InternalVulkan::create_image_view(
            device,
            depth_image,
            depth_format,
            ImageAspectFlags::DEPTH,
        );

        ImageData::create(depth_image, depth_image_memory, depth_image_view)
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

    pub fn draw_indexed_instanced(
        device: &Device,
        command_buffer: CommandBuffer,
        pipeline_layout: PipelineLayout,
        descriptor_set: DescriptorSet,
        render_system: &RenderSystem,
        mvps: &[ModelViewProjection],
        mvp_buffer: &StorageBufferObject,
    ) {
        mvp_buffer.update_data(mvps);
        unsafe {
            device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &[render_system.get_vertex_buffer()],
                &[0],
            );
            device.cmd_bind_index_buffer(
                command_buffer,
                render_system.get_index_buffer(),
                0,
                IndexType::UINT16,
            );

            device.cmd_bind_descriptor_sets(
                command_buffer,
                PipelineBindPoint::GRAPHICS,
                pipeline_layout,
                0,
                &[descriptor_set],
                &[],
            );

            device.cmd_draw_indexed(
                command_buffer,
                render_system.get_index_count(),
                mvps.len() as u32,
                0,
                0,
                0,
            );
        }
    }

    pub fn end_render_pass(device: &Device, command_buffer: CommandBuffer) {
        unsafe {
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer)
        }
        .expect("Failed to end command buffer!");
    }

    pub fn create_vertex_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        vertices: &[Vertex],
        command_pool: CommandPool,
        graphics_queue: Queue,
    ) -> (Buffer, DeviceMemory) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/01_Vertex_buffer_creation.html
        let buffer_size = size_of_val(vertices) as u64;

        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/02_Staging_buffer.html#_using_a_staging_buffer
        let (staging_buffer, staging_buffer_memory) = internal::InternalVulkan::create_buffer(
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

        let (vertex_buffer, vertex_buffer_memory) = internal::InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::VERTEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        internal::InternalVulkan::copy_buffer(
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

    pub fn create_index_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        indices: &[u16],
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Buffer, DeviceMemory) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/03_Index_buffer.html
        let buffer_size = size_of_val(indices) as u64;

        let (staging_buffer, staging_buffer_memory) = internal::InternalVulkan::create_buffer(
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
        .cast::<u16>();
        unsafe {
            copy_nonoverlapping(indices.as_ptr(), data, indices.len());
            device.unmap_memory(staging_buffer_memory);
        };

        let (index_buffer, index_buffer_memory) = internal::InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::INDEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        internal::InternalVulkan::copy_buffer(
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

        //pool
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
            .range((size_of::<ModelViewProjection>() * entity_count) as u64)];
        let descriptor_writes = [WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(0)
            .descriptor_type(DescriptorType::STORAGE_BUFFER)
            .buffer_info(&mvp_buffer_info)];
        unsafe {
            device.update_descriptor_sets(&descriptor_writes, &[]);
        };
    }

    pub fn create_texture(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ImageData {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_loading_an_image
        let (width, height) = image.dimensions();
        let image_size = u64::from(width * height * 4);
        let image_extent = Extent2D { width, height };

        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_staging_buffer
        let (staging_buffer, staging_buffer_memory) = internal::InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
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
        .expect("Failed to map memory for staging texture!")
        .cast::<u8>();
        let pixels = image.into_raw();
        unsafe {
            copy_nonoverlapping(pixels.as_ptr(), data, pixels.len());
            device.unmap_memory(staging_buffer_memory);
        };

        let format = Format::R8G8B8A8_SRGB;
        let (texture_image, image_memory) = internal::InternalVulkan::create_image(
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
        internal::InternalVulkan::transition_image_layout(
            device,
            graphics_queue,
            command_pool,
            texture_image,
            ImageLayout::UNDEFINED,
            ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        internal::InternalVulkan::copy_buffer_to_image(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            texture_image,
            image_extent,
        );
        internal::InternalVulkan::transition_image_layout(
            device,
            graphics_queue,
            command_pool,
            texture_image,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        };

        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/01_Image_view_and_sampler.html#_texture_image_view
        let image_view = internal::InternalVulkan::create_image_view(
            device,
            texture_image,
            format,
            ImageAspectFlags::COLOR,
        );

        ImageData::create(texture_image, image_memory, image_view)
    }

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

    pub fn create_ssbo(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        capacity: usize,
    ) -> (Buffer, DeviceMemory, *mut ModelViewProjection) {
        let buffer_size = (capacity * size_of::<ModelViewProjection>()) as u64;

        let (buffer, memory) = internal::InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::STORAGE_BUFFER,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );
        let mapped = unsafe { device.map_memory(memory, 0, buffer_size, MemoryMapFlags::empty()) }
            .expect("Failed to map memory for SSBO!")
            .cast::<ModelViewProjection>();

        (buffer, memory, mapped)
    }
}
