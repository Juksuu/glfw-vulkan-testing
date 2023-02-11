use std::{
    borrow::Cow,
    ffi::{c_void, CStr},
};

use ash::vk;

use crate::VulkanLibrary;

pub unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity, message_type, message_id_name, message_id_number, message,
    );

    vk::FALSE
}

pub fn create_debug_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(debug_callback))
        .build()
}

pub fn create_debug_info_c_void() -> *const c_void {
    &create_debug_info() as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void
}

pub struct DebugUtils {
    utils: ash::extensions::ext::DebugUtils,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugUtils {
    pub fn new(library: &VulkanLibrary, instance: &ash::Instance) -> Self {
        let utils = ash::extensions::ext::DebugUtils::new(&library.entry, &instance);

        let messenger = unsafe {
            utils
                .create_debug_utils_messenger(&create_debug_info(), None)
                .unwrap()
        };

        Self { utils, messenger }
    }
}

impl Drop for DebugUtils {
    fn drop(&mut self) {
        unsafe {
            self.utils
                .destroy_debug_utils_messenger(self.messenger, None)
        }
    }
}
