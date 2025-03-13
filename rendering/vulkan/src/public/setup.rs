use super::super::{
    internal::{external, Internal},
    structs::Vertex,
    Vulkan,
};

use ash::{
    khr::{surface, swapchain},
    vk::{
        AccessFlags, ApplicationInfo, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, ColorComponentFlags, CommandBuffer,
        CommandBufferAllocateInfo, CommandBufferLevel, CommandPool, CommandPoolCreateFlags,
        CommandPoolCreateInfo, CompareOp, CompositeAlphaFlagsKHR, CullModeFlags,
        DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCreateInfoEXT, DescriptorSetLayout, DeviceCreateInfo,
        DeviceQueueCreateInfo, DynamicState, Extent2D, Fence, FenceCreateFlags, FenceCreateInfo,
        Format, Framebuffer, FramebufferCreateInfo, FrontFace, GraphicsPipelineCreateInfo, Image,
        ImageAspectFlags, ImageLayout, ImageUsageFlags, ImageView, InstanceCreateInfo, LogicOp,
        Offset2D, PhysicalDevice, Pipeline, PipelineBindPoint, PipelineCache,
        PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
        PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineMultisampleStateCreateInfo,
        PipelineRasterizationStateCreateInfo, PipelineStageFlags,
        PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode,
        PrimitiveTopology, Rect2D, RenderPass, RenderPassCreateInfo, SampleCountFlags, Semaphore,
        SemaphoreCreateInfo, SharingMode, SubpassDependency, SubpassDescription, SurfaceKHR,
        SwapchainCreateInfoKHR, SwapchainKHR, Viewport, API_VERSION_1_3, SUBPASS_EXTERNAL,
    },
    Device, Entry, Instance,
};
use ash_window::{create_surface, enumerate_required_extensions};
use std::{array::from_ref, ffi::CString};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

impl Vulkan {
    #[must_use]
    pub fn create_vulkan_instance(
        entry: &Entry,
        window: &winit::window::Window,
        title: &str,
    ) -> Instance {
        let display_handle = window
            .display_handle()
            .expect("Failed to get display handle!")
            .as_raw();

        let application_name = CString::new(title).expect("Failed to create CString");
        let application_info = ApplicationInfo::default()
            .application_name(&application_name)
            .application_version(1)
            .api_version(API_VERSION_1_3);

        if cfg!(debug_assertions) {
            let mut extension_names = enumerate_required_extensions(display_handle)
                .expect("Failed to get required extensions!")
                .to_vec();
            extension_names.push(c"VK_EXT_debug_utils".as_ptr());

            let validation_layer = CString::new("VK_LAYER_KHRONOS_validation").unwrap();
            let layers = [validation_layer.as_ptr()];

            let mut debug_utils_info = DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | DebugUtilsMessageSeverityFlagsEXT::ERROR,
                )
                .message_type(
                    DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(external::debug_callback));

            let create_info = InstanceCreateInfo::default()
                .enabled_extension_names(&extension_names)
                .application_info(&application_info)
                .enabled_layer_names(&layers)
                .push_next(&mut debug_utils_info);

            unsafe { entry.create_instance(&create_info, None) }
                .expect("Failed to create Vulkan Instance!")
        } else {
            let extension_names = enumerate_required_extensions(display_handle)
                .expect("Failed to get required extensions!")
                .to_vec();

            let create_info = InstanceCreateInfo::default()
                .enabled_extension_names(&extension_names)
                .application_info(&application_info);

            unsafe { entry.create_instance(&create_info, None) }
                .expect("Failed to create Vulkan Instance!")
        }
    }

    #[must_use]
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
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to create a surface! Vulkan Error: {err},
                         Display handle: {display_handle:?}, Window handle: {window_handle:?}"
                    )
                });
        let surface_loader = surface::Instance::new(entry, instance);

        (surface, surface_loader)
    }

    #[must_use]
    pub fn find_physical_device(
        instance: &Instance,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, u32) {
        let devices = unsafe { instance.enumerate_physical_devices() }
            .expect("Failed to enumerate physical devices!");

        devices
            .iter()
            .find_map(|&device| {
                Internal::validate_physical_device(instance, device, surface, surface_loader)
            })
            .expect("Failed to find physical device that is valid!")
    }

    #[must_use]
    pub fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Device {
        let queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0]);
        let device_extensions = Internal::get_device_extensions(instance, physical_device);
        let vulkan13_features = Internal::get_13_features(instance, physical_device);
        let mut vulkan12_features = Internal::get_12_features(instance, physical_device);

        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(from_ref(&queue_create_info))
            .enabled_extension_names(&device_extensions)
            .enabled_features(&vulkan13_features)
            .push_next(&mut vulkan12_features);

        unsafe { instance.create_device(physical_device, &device_create_info, None) }
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to create logical device for physical device {physical_device:?}
                     and queue family index {queue_family_index}! Vulkan error: {err}"
                )
            })
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

        if surface_capabilities.max_image_count == 0 {
            min_image_count = min_image_count.max(3);
        } else {
            min_image_count = min_image_count.min(surface_capabilities.max_image_count);
        }

        let present_mode =
            Internal::get_swapchain_present_mode(physical_device, surface, surface_loader);
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

    #[must_use]
    pub fn create_image_views(
        swapchain_loader: &swapchain::Device,
        swapchain: SwapchainKHR,
        format: Format,
        device: &Device,
    ) -> (Vec<Image>, Vec<ImageView>) {
        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }
            .expect("Failed to get swapchain images!");

        let image_views: Vec<ImageView> = images
            .iter()
            .map(|&image| {
                Internal::create_image_view(device, image, format, ImageAspectFlags::COLOR)
            })
            .collect();

        (images, image_views)
    }

    #[must_use]
    pub fn create_render_pass(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        format: Format,
    ) -> RenderPass {
        const COLOR_ATTACHMENT_INDEX: u32 = 0;
        const DEPTH_ATTACHMENT_INDEX: u32 = 1;

        let depth_format = Internal::find_depth_format(instance, physical_device);
        let attachments = [
            AttachmentDescription::default()
                .format(format)
                .samples(SampleCountFlags::TYPE_1)
                .load_op(AttachmentLoadOp::CLEAR)
                .store_op(AttachmentStoreOp::STORE)
                .stencil_load_op(AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(AttachmentStoreOp::DONT_CARE)
                .initial_layout(ImageLayout::UNDEFINED)
                .final_layout(ImageLayout::PRESENT_SRC_KHR),
            AttachmentDescription::default()
                .format(depth_format)
                .samples(SampleCountFlags::TYPE_1)
                .load_op(AttachmentLoadOp::CLEAR)
                .store_op(AttachmentStoreOp::DONT_CARE)
                .stencil_load_op(AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(AttachmentStoreOp::DONT_CARE)
                .initial_layout(ImageLayout::UNDEFINED)
                .final_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL),
        ];

        let color_attachments = [AttachmentReference::default()
            .attachment(COLOR_ATTACHMENT_INDEX)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let depth_stencil_attachment = AttachmentReference::default()
            .attachment(DEPTH_ATTACHMENT_INDEX)
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

    #[must_use]
    pub fn create_graphics_pipeline(
        device: &Device,
        extent: Extent2D,
        render_pass: RenderPass,
        descriptor_set_layouts: &[DescriptorSetLayout],
        vertex_path: &str,
        fragment_path: &str,
    ) -> (PipelineLayout, Pipeline) {
        const LINE_WIDTH: f32 = 1.0;
        const SUBPASS_INDEX: u32 = 0;
        const BLEND_CONSTANTS: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let pipeline_layout = Internal::create_pipeline_layout(device, descriptor_set_layouts);
        let shaders = Internal::create_shader_modules(device, vertex_path, fragment_path);
        let shader_stages = Internal::create_shader_stages(&shaders);
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
            .line_width(LINE_WIDTH)
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
            .blend_constants(BLEND_CONSTANTS);

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
            .subpass(SUBPASS_INDEX)];
        let graphics_pipeline = unsafe {
            device.create_graphics_pipelines(PipelineCache::null(), &pipeline_infos, None)
        }
        .expect("Failed to create graphics pipeline!")[0];

        unsafe {
            device.destroy_shader_module(shaders.vertex_module, None);
            device.destroy_shader_module(shaders.fragment_module, None);
        };

        (pipeline_layout, graphics_pipeline)
    }

    #[must_use]
    pub fn create_framebuffers(
        device: &Device,
        render_pass: RenderPass,
        image_views: &[ImageView],
        depth_image_view: ImageView,
        extent: Extent2D,
    ) -> Vec<Framebuffer> {
        image_views
            .iter()
            .map(|&image_view| {
                let attachments = [image_view, depth_image_view];
                let framebuffer_create_info = FramebufferCreateInfo::default()
                    .render_pass(render_pass)
                    .attachments(&attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe { device.create_framebuffer(&framebuffer_create_info, None) }
                    .expect("Failed to create framebuffer!")
            })
            .collect()
    }

    #[must_use]
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

    #[must_use]
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
}
