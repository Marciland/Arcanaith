use super::{internal::InternalVulkan, ImageData};
use crate::{
    constants::{FRAGSHADER, TITLE, VERTSHADER},
    read_bytes_from_file, ModelViewProjection, Scene, UniformBufferObject, Vertex, VulkanWrapper,
};
use ash::{
    khr::{surface, swapchain},
    util::read_spv,
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
        PhysicalDeviceFeatures, Pipeline, PipelineBindPoint, PipelineCache,
        PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
        PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineStageFlags, PipelineVertexInputStateCreateInfo,
        PipelineViewportStateCreateInfo, PolygonMode, PresentModeKHR, PrimitiveTopology, Queue,
        Rect2D, RenderPass, RenderPassBeginInfo, RenderPassCreateInfo, SampleCountFlags, Sampler,
        SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode, Semaphore, SemaphoreCreateInfo,
        ShaderModuleCreateInfo, ShaderStageFlags, SharingMode, SubpassContents, SubpassDependency,
        SubpassDescription, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, Viewport,
        WriteDescriptorSet, API_VERSION_1_3, SUBPASS_EXTERNAL,
    },
    Device, Entry, Instance,
};
use ash_window::{create_surface, enumerate_required_extensions};
use image::{ImageBuffer, Rgba};
use std::{
    array::from_ref,
    ffi::{c_void, CStr},
    io::Cursor,
    mem,
    ptr::{self, copy_nonoverlapping},
};
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub trait Wrapper {
    fn create_vulkan_instance(entry: &Entry, window: &Window) -> Instance;
    fn create_surface(
        window: &Window,
        entry: &Entry,
        instance: &Instance,
    ) -> (SurfaceKHR, surface::Instance);
    fn find_physical_device(
        instance: &Instance,
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, u32);
    fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Device;
    fn create_swapchain(
        instance: &Instance,
        surface: SurfaceKHR,
        device: &Device,
        physical_device: PhysicalDevice,
        surface_loader: &surface::Instance,
    ) -> (SwapchainKHR, swapchain::Device, Format, Extent2D);
    fn create_image_views(
        swapchain_loader: &swapchain::Device,
        swapchain: SwapchainKHR,
        format: Format,
        device: &Device,
    ) -> (Vec<Image>, Vec<ImageView>);
    fn create_render_pass(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        format: Format,
    ) -> RenderPass;
    fn create_graphics_pipeline(
        device: &Device,
        extent: Extent2D,
        render_pass: RenderPass,
        descriptor_set_layouts: &[DescriptorSetLayout],
    ) -> (PipelineLayout, Pipeline);
    fn create_framebuffers(
        device: &Device,
        render_pass: RenderPass,
        image_views: &[ImageView],
        depth_image_view: ImageView,
        extent: Extent2D,
    ) -> Vec<Framebuffer>;
    fn create_command_buffer(
        device: &Device,
        queue_family_index: u32,
    ) -> (CommandPool, CommandBuffer);
    fn create_sync(device: &Device) -> (Semaphore, Semaphore, Fence);
    fn create_depth(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        extent: Extent2D,
    ) -> ImageData;
    fn begin_render_pass(
        device: &Device,
        framebuffers: &[Framebuffer],
        image_index: usize,
        command_buffer: CommandBuffer,
        extent: Extent2D,
        scene: &Scene,
    );
    fn draw_indexed_instanced(device: &Device, command_buffer: CommandBuffer, scene: &Scene);
    fn end_render_pass(device: &Device, command_buffer: CommandBuffer);
    fn create_vertex_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        vertices: &[Vertex],
        command_pool: CommandPool,
        graphics_queue: Queue,
    ) -> (Buffer, DeviceMemory);
    fn create_index_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        indices: &[u16],
        graphics_queue: Queue,
        command_pool: CommandPool,
    ) -> (Buffer, DeviceMemory);
    fn create_uniform_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        buffer_size: u64,
    ) -> UniformBufferObject;
    fn create_descriptor_pool(
        device: &Device,
        texture_count: u32,
    ) -> (DescriptorSetLayout, DescriptorPool);
    #[allow(clippy::too_many_arguments)]
    fn create_descriptor_set(
        device: &Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_layout: DescriptorSetLayout,
        texture_image_views: &[ImageView],
        texture_sampler: Sampler,
        object_count: usize,
        object_count_buffer: Buffer,
        mvp_buffer: Buffer,
    ) -> DescriptorSet;
    fn create_texture(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ImageData;
    fn create_texture_sampler(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
    ) -> Sampler;
}

impl Wrapper for VulkanWrapper {
    fn create_vulkan_instance(entry: &Entry, window: &Window) -> Instance {
        let extension_names =
            enumerate_required_extensions(window.display_handle().unwrap().as_raw())
                .unwrap()
                .to_vec();

        let application_name = TITLE.to_owned() + "\0";
        let application_info = ApplicationInfo::default()
            .application_name(CStr::from_bytes_with_nul(application_name.as_bytes()).unwrap())
            .application_version(1)
            .api_version(API_VERSION_1_3);

        let create_info = InstanceCreateInfo::default()
            .enabled_extension_names(&extension_names)
            .application_info(&application_info);

        // control validation layers from vulkan configurator!!
        unsafe { entry.create_instance(&create_info, None).unwrap() }
    }

    fn create_surface(
        window: &Window,
        entry: &Entry,
        instance: &Instance,
    ) -> (SurfaceKHR, surface::Instance) {
        let surface = unsafe {
            create_surface(
                entry,
                instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .unwrap()
        };
        let surface_loader = surface::Instance::new(entry, instance);
        (surface, surface_loader)
    }

    fn find_physical_device(
        instance: &Instance,
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, u32) {
        // TODO BestPractices-vkCreateDevice-physical-device-features-not-retrieved
        let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
        devices
            .iter()
            .find_map(|device| {
                let queue_family_properties =
                    unsafe { instance.get_physical_device_queue_family_properties(*device) };

                InternalVulkan::get_queue_family_index(
                    surface,
                    surface_loader,
                    *device,
                    queue_family_properties,
                )
            })
            .unwrap()
    }

    fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Device {
        let queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0]);

        let device_features = PhysicalDeviceFeatures::default().sampler_anisotropy(true);

        let device_extensions = [swapchain::NAME.as_ptr()];

        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(from_ref(&queue_create_info))
            .enabled_extension_names(&device_extensions)
            .enabled_features(&device_features);

        unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap()
        }
    }

    fn create_swapchain(
        instance: &Instance,
        surface: SurfaceKHR,
        device: &Device,
        physical_device: PhysicalDevice,
        surface_loader: &surface::Instance,
    ) -> (SwapchainKHR, swapchain::Device, Format, Extent2D) {
        let swapchain_loader = swapchain::Device::new(instance, device);

        let (surface_capabilities, surface_format) = unsafe {
            let surface_capabilities = surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)
                .unwrap();

            let surface_format = surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap()[0]; /* in most cases it’s okay to just settle with the first format that is specified */

            (surface_capabilities, surface_format)
        };

        let mut min_image_count = surface_capabilities.min_image_count + 1;
        if min_image_count > surface_capabilities.max_image_count {
            min_image_count = surface_capabilities.max_image_count
        }

        let present_mode = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap()
                .iter()
                .cloned()
                .find(|&mode| mode == PresentModeKHR::MAILBOX)
                .unwrap_or(PresentModeKHR::FIFO)
        };

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

        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap()
        };

        (swapchain, swapchain_loader, format, extent)
    }

    fn create_image_views(
        swapchain_loader: &swapchain::Device,
        swapchain: SwapchainKHR,
        format: Format,
        device: &Device,
    ) -> (Vec<Image>, Vec<ImageView>) {
        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };
        let mut image_views: Vec<ImageView> = Vec::with_capacity(images.len());

        for image in &images {
            image_views.push(InternalVulkan::create_image_view(
                device,
                *image,
                format,
                ImageAspectFlags::COLOR,
            ))
        }
        (images, image_views)
    }

    fn create_render_pass(
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
            .format(InternalVulkan::find_depth_format(instance, physical_device))
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

        unsafe { device.create_render_pass(&render_pass_info, None).unwrap() }
    }

    fn create_graphics_pipeline(
        device: &Device,
        extent: Extent2D,
        render_pass: RenderPass,
        descriptor_set_layouts: &[DescriptorSetLayout],
    ) -> (PipelineLayout, Pipeline) {
        let (vert_shader_module, frag_shader_module) = unsafe {
            let vert_shader_module = device
                .create_shader_module(
                    &ShaderModuleCreateInfo::default().code(
                        &read_spv(&mut Cursor::new(&read_bytes_from_file(VERTSHADER))).unwrap(),
                    ),
                    None,
                )
                .unwrap();
            let frag_shader_module = device
                .create_shader_module(
                    &ShaderModuleCreateInfo::default().code(
                        &read_spv(&mut Cursor::new(&read_bytes_from_file(FRAGSHADER))).unwrap(),
                    ),
                    None,
                )
                .unwrap();
            (vert_shader_module, frag_shader_module)
        };

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

        let pipeline_layout_info =
            PipelineLayoutCreateInfo::default().set_layouts(descriptor_set_layouts);

        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&pipeline_layout_info, None)
                .unwrap()
        };

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

        let graphics_pipeline = unsafe {
            let graphics_pipeline = device
                .create_graphics_pipelines(PipelineCache::null(), &pipeline_infos, None)
                .unwrap()[0];

            device.destroy_shader_module(vert_shader_module, None);
            device.destroy_shader_module(frag_shader_module, None);

            graphics_pipeline
        };

        (pipeline_layout, graphics_pipeline)
    }

    fn create_framebuffers(
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
            let framebuffer = unsafe {
                device
                    .create_framebuffer(&framebuffer_create_info, None)
                    .unwrap()
            };
            framebuffers.push(framebuffer);
        }

        framebuffers
    }

    fn create_command_buffer(
        device: &Device,
        queue_family_index: u32,
    ) -> (CommandPool, CommandBuffer) {
        let pool_create_info = CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(CommandPoolCreateFlags::empty());
        let command_pool = unsafe { device.create_command_pool(&pool_create_info, None).unwrap() };

        let allocation_info = CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let command_buffer =
            unsafe { device.allocate_command_buffers(&allocation_info).unwrap()[0] };

        (command_pool, command_buffer)
    }

    fn create_sync(device: &Device) -> (Semaphore, Semaphore, Fence) {
        let semaphore_create_info = SemaphoreCreateInfo::default();
        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

        unsafe {
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
    }

    fn create_depth(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        extent: Extent2D,
    ) -> ImageData {
        // https://docs.vulkan.org/tutorial/latest/07_Depth_buffering.html#_depth_image_and_view
        let depth_format = InternalVulkan::find_depth_format(instance, physical_device);
        let (depth_image, depth_image_memory) = InternalVulkan::create_image(
            instance,
            physical_device,
            device,
            extent,
            depth_format,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );
        let depth_image_view = InternalVulkan::create_image_view(
            device,
            depth_image,
            depth_format,
            ImageAspectFlags::DEPTH,
        );

        ImageData {
            image: depth_image,
            memory: depth_image_memory,
            view: depth_image_view,
        }
    }

    fn begin_render_pass(
        device: &Device,
        framebuffers: &[Framebuffer],
        image_index: usize,
        command_buffer: CommandBuffer,
        extent: Extent2D,
        scene: &Scene,
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
            .render_pass(scene.get_render_pass())
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

        unsafe {
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .unwrap();

            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                SubpassContents::INLINE,
            );

            device.cmd_set_viewport(command_buffer, 0, &viewports);
            device.cmd_set_scissor(command_buffer, 0, &scissors);

            device.cmd_bind_pipeline(
                command_buffer,
                PipelineBindPoint::GRAPHICS,
                scene.get_pipeline(),
            );
        };
    }

    fn draw_indexed_instanced(device: &Device, command_buffer: CommandBuffer, scene: &Scene) {
        unsafe {
            copy_nonoverlapping(
                scene.get_mvp_data().as_ptr(),
                scene.get_mvp_uniform().get_mapped() as *mut ModelViewProjection,
                scene.get_mvp_data().len(),
            );

            device.cmd_bind_vertex_buffers(command_buffer, 0, &[scene.get_vertex_buffer()], &[0]);
            device.cmd_bind_index_buffer(
                command_buffer,
                scene.get_index_buffer(),
                0,
                IndexType::UINT16,
            );

            device.cmd_bind_descriptor_sets(
                command_buffer,
                PipelineBindPoint::GRAPHICS,
                scene.get_pipeline_layout(),
                0,
                &[scene.get_descriptor_set()],
                &[],
            );

            let objects = scene.get_objects();
            ptr::write(
                scene.get_objects_uniform().get_mapped() as *mut i32,
                objects.len() as i32,
            );

            device.cmd_draw_indexed(
                command_buffer,
                scene.get_index_count(),
                objects.len() as u32,
                0,
                0,
                0,
            );
        }
    }

    fn end_render_pass(device: &Device, command_buffer: CommandBuffer) {
        unsafe {
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer).unwrap();
        }
    }

    fn create_vertex_buffer(
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

        unsafe {
            let data: *mut c_void = device
                .map_memory(
                    staging_buffer_memory,
                    0,
                    buffer_size,
                    MemoryMapFlags::empty(),
                )
                .unwrap();
            copy_nonoverlapping(vertices.as_ptr(), data as *mut Vertex, vertices.len());
            device.unmap_memory(staging_buffer_memory);
        };

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

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        };

        (vertex_buffer, vertex_buffer_memory)
    }

    fn create_index_buffer(
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

        unsafe {
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
        };

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

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        };

        (index_buffer, index_buffer_memory)
    }

    fn create_uniform_buffer(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        buffer_size: u64,
    ) -> UniformBufferObject {
        let (buffer, memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::UNIFORM_BUFFER,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );
        let mapped = unsafe {
            device
                .map_memory(memory, 0, buffer_size, MemoryMapFlags::empty())
                .unwrap()
        };

        UniformBufferObject::create(buffer, memory, mapped)
    }

    fn create_descriptor_pool(
        device: &Device,
        texture_count: u32,
    ) -> (DescriptorSetLayout, DescriptorPool) {
        // https://docs.vulkan.org/tutorial/latest/05_Uniform_buffers/00_Descriptor_set_layout_and_buffer.html#_descriptor_set_layout
        let ubo_layout_binding = DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .stage_flags(ShaderStageFlags::VERTEX);

        let object_count_binding = DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .stage_flags(ShaderStageFlags::VERTEX);

        let sampler_layout_binding = DescriptorSetLayoutBinding::default()
            .binding(2)
            .descriptor_count(texture_count)
            .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
            .stage_flags(ShaderStageFlags::FRAGMENT);

        let bindings = [
            ubo_layout_binding,
            object_count_binding,
            sampler_layout_binding,
        ];
        let layout_create_info = DescriptorSetLayoutCreateInfo::default().bindings(&bindings);

        let descriptor_set_layout = unsafe {
            device
                .create_descriptor_set_layout(&layout_create_info, None)
                .unwrap()
        };

        // https://docs.vulkan.org/tutorial/latest/05_Uniform_buffers/01_Descriptor_pool_and_sets.html#_introduction
        let pool_sizes = [
            DescriptorPoolSize::default()
                .descriptor_count(2)
                .ty(DescriptorType::UNIFORM_BUFFER),
            DescriptorPoolSize::default()
                .descriptor_count(texture_count) // assuming each object has its own texture
                .ty(DescriptorType::COMBINED_IMAGE_SAMPLER),
        ];
        let pool_info = DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(1);

        let descriptor_pool = unsafe { device.create_descriptor_pool(&pool_info, None).unwrap() };

        (descriptor_set_layout, descriptor_pool)
    }

    fn create_descriptor_set(
        device: &Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_layout: DescriptorSetLayout,
        texture_image_views: &[ImageView],
        texture_sampler: Sampler,
        object_count: usize,
        object_count_buffer: Buffer,
        mvp_buffer: Buffer,
    ) -> DescriptorSet {
        let descriptor_set_layouts = &[descriptor_set_layout];
        let descriptor_allocation_info = DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(descriptor_set_layouts);
        let descriptor_set = unsafe {
            device
                .allocate_descriptor_sets(&descriptor_allocation_info)
                .unwrap()[0]
        };

        let mvp_buffer_info = [DescriptorBufferInfo::default()
            .buffer(mvp_buffer)
            .offset(0)
            .range((mem::size_of::<ModelViewProjection>() * object_count) as u64)];
        let object_count_infos = [DescriptorBufferInfo::default()
            .buffer(object_count_buffer)
            .offset(0)
            .range(mem::size_of::<i32>() as u64)];
        let mut image_infos: Vec<DescriptorImageInfo> =
            Vec::with_capacity(texture_image_views.len());
        for &image_view in texture_image_views.iter() {
            image_infos.push(
                DescriptorImageInfo::default()
                    .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .image_view(image_view)
                    .sampler(texture_sampler),
            );
        }

        let descriptor_writes = [
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(0)
                .descriptor_type(DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&mvp_buffer_info),
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(1)
                .descriptor_type(DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&object_count_infos),
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(2)
                .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(&image_infos),
        ];

        unsafe {
            device.update_descriptor_sets(&descriptor_writes, &[]);
        };

        descriptor_set
    }

    fn create_texture(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        graphics_queue: Queue,
        command_pool: CommandPool,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ImageData {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/00_Images.html#_loading_an_image
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

        unsafe {
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
        };

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

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        };

        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/01_Image_view_and_sampler.html#_texture_image_view
        let image_view = InternalVulkan::create_image_view(
            device,
            texture_image,
            format,
            ImageAspectFlags::COLOR,
        );

        ImageData {
            image: texture_image,
            memory: image_memory,
            view: image_view,
        }
    }

    fn create_texture_sampler(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
    ) -> Sampler {
        // https://docs.vulkan.org/tutorial/latest/06_Texture_mapping/01_Image_view_and_sampler.html#_samplers
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

        unsafe { device.create_sampler(&sampler_create_info, None).unwrap() }
    }
}
