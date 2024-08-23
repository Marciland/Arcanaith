use ash::{
    vk::{ApplicationInfo, InstanceCreateInfo, PhysicalDevice, QueueFlags, API_VERSION_1_3},
    Entry, Instance,
};
use ash_window::enumerate_required_extensions;
use std::{ffi::CStr, os::raw::c_char};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

use crate::constants::TITLE;

pub unsafe fn create_vulkan_instance(entry: &Entry, window: &Window) -> Instance {
    // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/01_Instance.html
    let extension_names = enumerate_required_extensions(window.display_handle().unwrap().as_raw())
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
    return entry.create_instance(&create_info, None).unwrap();

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

pub unsafe fn find_physical_device(instance: &Instance) -> PhysicalDevice {
    let devices = instance.enumerate_physical_devices().unwrap();
    let (physical_device, _queue_family_index) = devices
        .iter()
        .find_map(|device| {
            instance
                .get_physical_device_queue_family_properties(*device)
                .iter()
                .enumerate()
                .find_map(|(index, info)| {
                    let supports_graphic_and_surface =
                        info.queue_flags.contains(QueueFlags::GRAPHICS); // surface check?
                    if supports_graphic_and_surface {
                        Some((*device, index))
                    } else {
                        None
                    }
                })
        })
        .unwrap();
    physical_device
}
