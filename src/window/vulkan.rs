use ash::{
    khr::surface,
    vk::{
        ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, InstanceCreateInfo,
        PhysicalDevice, PhysicalDeviceFeatures, QueueFlags, SurfaceKHR, API_VERSION_1_3,
    },
    Device, Entry, Instance,
};
use ash_window::enumerate_required_extensions;
#[cfg(debug_assertions)]
use std::os::raw::c_char;
use std::{array::from_ref, ffi::CStr};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

use crate::constants::TITLE;

pub struct VulkanWrapper;

pub trait VulkanInterface {
    unsafe fn create_vulkan_instance(entry: &Entry, window: &Window) -> Instance;
    unsafe fn find_physical_device(
        instance: &Instance,
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, usize);
    unsafe fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Device;
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

    unsafe fn find_physical_device(
        instance: &Instance,
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (PhysicalDevice, usize) {
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
                            Some((*device, index))
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

        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(from_ref(&queue_create_info))
            .enabled_features(&device_features);

        instance
            .create_device(physical_device, &device_create_info, None)
            .unwrap()
    }
}
