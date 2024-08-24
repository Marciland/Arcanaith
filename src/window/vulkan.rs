use ash::{
    khr::{surface, swapchain},
    vk::{
        ApplicationInfo, ComponentMapping, ComponentSwizzle, CompositeAlphaFlagsKHR,
        DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, Format, Image, ImageAspectFlags,
        ImageSubresourceRange, ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType,
        InstanceCreateInfo, PhysicalDevice, PhysicalDeviceFeatures, PresentModeKHR, QueueFlags,
        SharingMode, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, API_VERSION_1_3,
    },
    Device, Entry, Instance,
};
use ash_window::{create_surface, enumerate_required_extensions};
#[cfg(debug_assertions)]
use std::os::raw::c_char;
use std::{array::from_ref, ffi::CStr};
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use crate::constants::TITLE;

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
        images: &[Image],
        format: Format,
        device: &Device,
    ) -> Vec<ImageView>;
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

        #[cfg(not(debug_assertions))]
        {
            entry.create_instance(&create_info, None).unwrap()
        }

        #[cfg(debug_assertions)]
        {
            let layers_names: Vec<*const c_char> = [CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )]
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

            entry
                .create_instance(&create_info.enabled_layer_names(&layers_names), None)
                .unwrap()
        }
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
        images: &[Image],
        format: Format,
        device: &Device,
    ) -> Vec<ImageView> {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/01_Presentation/02_Image_views.html
        let mut image_views: Vec<ImageView> = Vec::new();

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

        for image in images {
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
        image_views
    }
}
