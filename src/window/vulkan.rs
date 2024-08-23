use ash::{vk::InstanceCreateInfo, Entry, Instance};
use ash_window::enumerate_required_extensions;
use std::{ffi, os::raw::c_char};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

pub unsafe fn create_vulkan_instance(entry: &Entry, window: &Window, debug: bool) -> Instance {
    let extension_names = enumerate_required_extensions(window.display_handle().unwrap().as_raw())
        .unwrap()
        .to_vec();
    let create_info = InstanceCreateInfo::default().enabled_extension_names(&extension_names);

    if debug {
        let layers_names: Vec<*const c_char> = [ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_LAYER_KHRONOS_validation\0",
        )]
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();

        return entry
            .create_instance(&create_info.enabled_layer_names(&layers_names), None)
            .unwrap();
    }

    entry.create_instance(&create_info, None).unwrap()
}
