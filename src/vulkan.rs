use super::vertex::Vertex;
use ash::{
    khr::{surface, swapchain},
    util::read_spv,
    vk::{
        AccessFlags, ApplicationInfo, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, Buffer, BufferCopy, BufferCreateInfo,
        BufferUsageFlags, ClearColorValue, ClearValue, ColorComponentFlags, CommandBuffer,
        CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo,
        ComponentMapping, ComponentSwizzle, CompositeAlphaFlagsKHR, CullModeFlags,
        DeviceCreateInfo, DeviceMemory, DeviceQueueCreateInfo, DynamicState, Extent2D, Fence,
        FenceCreateFlags, FenceCreateInfo, Format, Framebuffer, FramebufferCreateInfo, FrontFace,
        GraphicsPipelineCreateInfo, Image, ImageAspectFlags, ImageLayout, ImageSubresourceRange,
        ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, IndexType,
        InstanceCreateInfo, LogicOp, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags,
        MemoryRequirements, Offset2D, PhysicalDevice, PhysicalDeviceFeatures, Pipeline,
        PipelineBindPoint, PipelineCache, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo,
        PipelineDynamicStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PipelineLayout,
        PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo,
        PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateInfo, PipelineStageFlags,
        PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode,
        PresentModeKHR, PrimitiveTopology, Queue, QueueFlags, Rect2D, RenderPass,
        RenderPassBeginInfo, RenderPassCreateInfo, SampleCountFlags, Semaphore,
        SemaphoreCreateInfo, ShaderModuleCreateInfo, ShaderStageFlags, SharingMode, SubmitInfo,
        SubpassContents, SubpassDependency, SubpassDescription, SurfaceKHR, SwapchainCreateInfoKHR,
        SwapchainKHR, Viewport, API_VERSION_1_3, SUBPASS_EXTERNAL,
    },
    Device, Entry, Instance,
};
use ash_window::{create_surface, enumerate_required_extensions};
use std::{array::from_ref, ffi::CStr, io::Cursor, mem, ptr::copy_nonoverlapping};
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use crate::{
    constants::{FRAGSHADER, TITLE, VERTSHADER},
    read_bytes_from_file,
};

pub struct VulkanWrapper;

pub trait VulkanInterface {
    unsafe fn create_vulkan_instance(entry: &Entry, window: &Window) -> Instance;
    unsafe fn create_surface(
        window: &Window,
        entry: &Entry,
        instance: &Instance,
    ) -> (SurfaceKHR, surface::Instance);
    unsafe fn find_physical_device(
        instance: &Instance,
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, u32);
    unsafe fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Device;
    unsafe fn create_swapchain(
        instance: &Instance,
        surface: SurfaceKHR,
        device: &Device,
        physical_device: PhysicalDevice,
        surface_loader: &surface::Instance,
    ) -> (SwapchainKHR, swapchain::Device, Format, Extent2D);
    unsafe fn create_image_views(
        swapchain_loader: &swapchain::Device,
        swapchain: SwapchainKHR,
        format: Format,
        device: &Device,
    ) -> (Vec<Image>, Vec<ImageView>);
    unsafe fn create_render_pass(device: &Device, format: Format) -> RenderPass;
    unsafe fn create_graphics_pipeline(
        device: &Device,
        extent: Extent2D,
        render_pass: RenderPass,
    ) -> (PipelineLayout, Pipeline);
    unsafe fn create_framebuffers(
        device: &Device,
        render_pass: RenderPass,
        image_views: &[ImageView],
        extent: Extent2D,
    ) -> Vec<Framebuffer>;
    unsafe fn create_command_pool(device: &Device, queue_family_index: u32) -> CommandPool;
    unsafe fn create_command_buffer(device: &Device, command_pool: CommandPool) -> CommandBuffer;
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
    );
    unsafe fn create_sync(device: &Device) -> (Semaphore, Semaphore, Fence);
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
        indices: Vec<u16>,
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Buffer, DeviceMemory);
}

impl VulkanWrapper {
    unsafe fn create_buffer(
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
            .memory_type_index(VulkanWrapper::find_memory_type(
                instance,
                physical_device,
                memory_requirements,
                memory_properties,
            ));
        let buffer_memory = device.allocate_memory(&allocation_info, None).unwrap();

        device.bind_buffer_memory(buffer, buffer_memory, 0).unwrap();

        (buffer, buffer_memory)
    }

    unsafe fn copy_buffer(
        device: &Device,
        command_pool: CommandPool,
        graphics_queue: Queue,
        src: Buffer,
        dst: Buffer,
        size: u64,
    ) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/02_Staging_buffer.html#_conclusion
        let allocation_info = CommandBufferAllocateInfo::default()
            .level(CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);
        let command_buffers = device.allocate_command_buffers(&allocation_info).unwrap();

        device
            .begin_command_buffer(
                command_buffers[0],
                &CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT),
            )
            .unwrap();
        device.cmd_copy_buffer(
            command_buffers[0],
            src,
            dst,
            &[BufferCopy::default().src_offset(0).dst_offset(0).size(size)],
        );
        device.end_command_buffer(command_buffers[0]).unwrap();

        device
            .queue_submit(
                graphics_queue,
                &[SubmitInfo::default().command_buffers(&command_buffers)],
                Fence::null(),
            )
            .unwrap();
        device.queue_wait_idle(graphics_queue).unwrap();
        device.free_command_buffers(command_pool, &command_buffers)
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
}

impl VulkanInterface for VulkanWrapper {
    unsafe fn create_vulkan_instance(entry: &Entry, window: &Window) -> Instance {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/01_Instance.html
        let extension_names =
            enumerate_required_extensions(window.display_handle().unwrap().as_raw())
                .unwrap()
                .to_vec();

        let application_info = ApplicationInfo::default()
            .application_name(CStr::from_bytes_with_nul_unchecked(TITLE.as_bytes()))
            .application_version(1)
            .api_version(API_VERSION_1_3);

        let create_info = InstanceCreateInfo::default()
            .enabled_extension_names(&extension_names)
            .application_info(&application_info);

        // control validation layers from vulkan configurator!!
        entry.create_instance(&create_info, None).unwrap()
    }

    unsafe fn create_surface(
        window: &Window,
        entry: &Entry,
        instance: &Instance,
    ) -> (SurfaceKHR, surface::Instance) {
        let surface = create_surface(
            entry,
            instance,
            window.display_handle().unwrap().as_raw(),
            window.window_handle().unwrap().as_raw(),
            None,
        )
        .unwrap();
        let surface_loader = surface::Instance::new(entry, instance);
        (surface, surface_loader)
    }

    unsafe fn find_physical_device(
        instance: &Instance,
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, u32) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/03_Physical_devices_and_queue_families.html
        // TODO BestPractices-vkCreateDevice-physical-device-features-not-retrieved
        instance
            .enumerate_physical_devices()
            .unwrap()
            .iter()
            .find_map(|device| {
                instance
                    .get_physical_device_queue_family_properties(*device)
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)| {
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(QueueFlags::GRAPHICS)
                                && surface_loader
                                    .get_physical_device_surface_support(
                                        *device,
                                        index as u32,
                                        *surface,
                                    )
                                    .unwrap();
                        if supports_graphic_and_surface {
                            Some((*device, index as u32))
                        } else {
                            None
                        }
                    })
            })
            .unwrap()
    }

    unsafe fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Device {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/04_Logical_device_and_queues.html
        let queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0]);

        let device_features = PhysicalDeviceFeatures::default();

        let device_extensions = [swapchain::NAME.as_ptr()];

        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(from_ref(&queue_create_info))
            .enabled_extension_names(&device_extensions)
            .enabled_features(&device_features);

        instance
            .create_device(physical_device, &device_create_info, None)
            .unwrap()
    }

    unsafe fn create_swapchain(
        instance: &Instance,
        surface: SurfaceKHR,
        device: &Device,
        physical_device: PhysicalDevice,
        surface_loader: &surface::Instance,
    ) -> (SwapchainKHR, swapchain::Device, Format, Extent2D) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/01_Presentation/01_Swap_chain.html#_creating_the_swap_chain
        let swapchain_loader = swapchain::Device::new(instance, device);

        let surface_capabilities = surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface)
            .unwrap();

        let surface_format = surface_loader
            .get_physical_device_surface_formats(physical_device, surface)
            .unwrap()[0]; /* in most cases it’s okay to just settle with the first format that is specified */

        let mut min_image_count = surface_capabilities.min_image_count + 1;
        if min_image_count > surface_capabilities.max_image_count {
            min_image_count = surface_capabilities.max_image_count
        }

        let present_mode = surface_loader
            .get_physical_device_surface_present_modes(physical_device, surface)
            .unwrap()
            .iter()
            .cloned()
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

        let swapchain = swapchain_loader
            .create_swapchain(&swapchain_create_info, None)
            .unwrap();

        (swapchain, swapchain_loader, format, extent)
    }

    unsafe fn create_image_views(
        swapchain_loader: &swapchain::Device,
        swapchain: SwapchainKHR,
        format: Format,
        device: &Device,
    ) -> (Vec<Image>, Vec<ImageView>) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/01_Presentation/02_Image_views.html
        let mut image_views: Vec<ImageView> = Vec::new();
        let images = swapchain_loader.get_swapchain_images(swapchain).unwrap();

        let components = ComponentMapping::default()
            .r(ComponentSwizzle::IDENTITY)
            .g(ComponentSwizzle::IDENTITY)
            .b(ComponentSwizzle::IDENTITY)
            .a(ComponentSwizzle::IDENTITY);

        let subresource_range = ImageSubresourceRange::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        for image in &images {
            let image_view_create_info = ImageViewCreateInfo::default()
                .image(*image)
                .view_type(ImageViewType::TYPE_2D)
                .format(format)
                .components(components)
                .subresource_range(subresource_range);

            let image_view = device
                .create_image_view(&image_view_create_info, None)
                .unwrap();
            image_views.push(image_view)
        }
        (images, image_views)
    }

    unsafe fn create_render_pass(device: &Device, format: Format) -> RenderPass {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/02_Graphics_pipeline_basics/03_Render_passes.html
        let color_attachments = [AttachmentDescription::default()
            .format(format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR)];

        let color_attachment_references = [AttachmentReference::default()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let subpasses = [SubpassDescription::default()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_references)];

        let dependencies = [SubpassDependency::default()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlags::empty())
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)];

        let render_pass_info = RenderPassCreateInfo::default()
            .attachments(&color_attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        device.create_render_pass(&render_pass_info, None).unwrap()
    }

    unsafe fn create_graphics_pipeline(
        device: &Device,
        extent: Extent2D,
        render_pass: RenderPass,
    ) -> (PipelineLayout, Pipeline) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/02_Graphics_pipeline_basics/00_Introduction.html
        let vert_shader_module = device
            .create_shader_module(
                &ShaderModuleCreateInfo::default()
                    .code(&read_spv(&mut Cursor::new(&read_bytes_from_file(VERTSHADER))).unwrap()),
                None,
            )
            .unwrap();
        let frag_shader_module = device
            .create_shader_module(
                &ShaderModuleCreateInfo::default()
                    .code(&read_spv(&mut Cursor::new(&read_bytes_from_file(FRAGSHADER))).unwrap()),
                None,
            )
            .unwrap();

        let shader_stages = [
            PipelineShaderStageCreateInfo::default()
                .stage(ShaderStageFlags::VERTEX)
                .module(vert_shader_module)
                .name(CStr::from_bytes_with_nul("main\0".as_bytes()).unwrap()),
            PipelineShaderStageCreateInfo::default()
                .stage(ShaderStageFlags::FRAGMENT)
                .module(frag_shader_module)
                .name(CStr::from_bytes_with_nul("main\0".as_bytes()).unwrap()),
        ];

        let dynamic_state = PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&[DynamicState::SCISSOR, DynamicState::VIEWPORT]);

        let binding_descriptions = [Vertex::get_binding_description()];

        let attribute_descriptions = Vertex::get_attribute_descriptions();

        let vertex_input_info = PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions);

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

        let depth_stencil_state = PipelineDepthStencilStateCreateInfo::default();

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

        let pipeline_layout_info = PipelineLayoutCreateInfo::default()
            .set_layouts(&[])
            .push_constant_ranges(&[]);

        let pipeline_layout = device
            .create_pipeline_layout(&pipeline_layout_info, None)
            .unwrap();

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/02_Graphics_pipeline_basics/04_Conclusion.html
        let pipeline_infos = [GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
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

        let graphics_pipeline = device
            .create_graphics_pipelines(PipelineCache::null(), &pipeline_infos, None)
            .unwrap()[0];

        device.destroy_shader_module(vert_shader_module, None);
        device.destroy_shader_module(frag_shader_module, None);

        (pipeline_layout, graphics_pipeline)
    }

    unsafe fn create_framebuffers(
        device: &Device,
        render_pass: RenderPass,
        image_views: &[ImageView],
        extent: Extent2D,
    ) -> Vec<Framebuffer> {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/00_Framebuffers.html
        let mut framebuffers: Vec<Framebuffer> = Vec::new();

        for image_view in image_views {
            let attachments = [*image_view];
            let framebuffer_create_info = FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);
            let framebuffer: Framebuffer = device
                .create_framebuffer(&framebuffer_create_info, None)
                .unwrap();
            framebuffers.push(framebuffer);
        }

        framebuffers
    }

    unsafe fn create_command_pool(device: &Device, queue_family_index: u32) -> CommandPool {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html
        // TODO reset pool instead
        let pool_create_info = CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        device.create_command_pool(&pool_create_info, None).unwrap()
    }

    unsafe fn create_command_buffer(device: &Device, command_pool: CommandPool) -> CommandBuffer {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_command_buffer_allocation
        let allocation_info = CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        device.allocate_command_buffers(&allocation_info).unwrap()[0]
    }

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
    ) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_command_buffer_recording
        let begin_info = CommandBufferBeginInfo::default();

        device
            .begin_command_buffer(command_buffer, &begin_info)
            .unwrap();

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_starting_a_render_pass
        let clear_colors = [ClearValue {
            color: ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];

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

        device.cmd_draw_indexed(command_buffer, index_count, 1, 0, 0, 0);

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/01_Command_buffers.html#_finishing_up
        device.cmd_end_render_pass(command_buffer);
        device.end_command_buffer(command_buffer).unwrap();
    }

    unsafe fn create_sync(device: &Device) -> (Semaphore, Semaphore, Fence) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_creating_the_synchronization_objects
        let semaphore_create_info = SemaphoreCreateInfo::default();
        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

        (
            device
                .create_semaphore(&semaphore_create_info, None)
                .unwrap(),
            device
                .create_semaphore(&semaphore_create_info, None)
                .unwrap(),
            device.create_fence(&fence_create_info, None).unwrap(),
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
        let (staging_buffer, staging_buffer_memory) = VulkanWrapper::create_buffer(
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
        copy_nonoverlapping(vertices.as_ptr(), data as *mut Vertex, buffer_size as usize);
        device.unmap_memory(staging_buffer_memory);

        let (vertex_buffer, vertex_buffer_memory) = VulkanWrapper::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::VERTEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        VulkanWrapper::copy_buffer(
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
        indices: Vec<u16>,
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Buffer, DeviceMemory) {
        // https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/03_Index_buffer.html
        let buffer_size = mem::size_of_val(&indices) as u64;

        let (staging_buffer, staging_buffer_memory) = VulkanWrapper::create_buffer(
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
        copy_nonoverlapping(indices.as_ptr(), data as *mut u16, buffer_size as usize);
        device.unmap_memory(staging_buffer_memory);

        let (index_buffer, index_buffer_memory) = VulkanWrapper::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::INDEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        VulkanWrapper::copy_buffer(
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
}
