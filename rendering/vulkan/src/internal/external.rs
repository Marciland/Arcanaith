use ash::vk::{
    Bool32, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCallbackDataEXT, FALSE,
};
use std::ffi::{c_void, CStr};

pub unsafe extern "system" fn debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> Bool32 {
    let message = CStr::from_ptr((*p_callback_data).p_message).to_string_lossy();
    let severity = match message_severity {
        DebugUtilsMessageSeverityFlagsEXT::WARNING => "WARNING",
        DebugUtilsMessageSeverityFlagsEXT::ERROR => "ERROR",
        _ => "UNKNOWN",
    };
    let message_type = match message_type {
        DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
        DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
        _ => "UNKNOWN",
    };

    println!("[VULKAN]: {severity} - {message_type}: {message}");
    FALSE
}
