use ash::vk;
use std::{
    ffi::{c_void, CStr, CString},
    ptr,
};

use super::constants::{USE_VALIDATION_LAYERS, VALIDATION_LAYERS};

fn check_validation_layer_support(entry: &ash::Entry) -> bool {
    let layer_properties = entry.enumerate_instance_layer_properties().unwrap();

    if layer_properties.len() <= 0 {
        eprintln!("No available layers");
        return false;
    }

    let layer_properties_str: Vec<&str> = layer_properties
        .iter()
        .map(|l| unsafe { CStr::from_ptr(l.layer_name.as_ptr()).to_str().unwrap() })
        .collect();

    VALIDATION_LAYERS
        .iter()
        .all(|item| layer_properties_str.contains(item))
}

pub fn create_instance(entry: &ash::Entry, window: &crate::Window) -> ash::Instance {
    if USE_VALIDATION_LAYERS && !check_validation_layer_support(entry) {
        panic!("Validation layers requested, but not available");
    }

    let app_info = vk::ApplicationInfo {
        api_version: vk::API_VERSION_1_3,
        ..Default::default()
    };

    let mut extensions: Vec<*const i8> =
        ash_window::enumerate_required_extensions(window.get_raw_display_handle())
            .unwrap()
            .to_vec();

    extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());

    let layer_names_cstring: Vec<CString> = VALIDATION_LAYERS
        .iter()
        .map(|l| CString::new(l.to_owned()).unwrap())
        .collect();
    let layer_names: Vec<*const i8> = layer_names_cstring.iter().map(|l| l.as_ptr()).collect();

    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
        enabled_layer_count: if USE_VALIDATION_LAYERS {
            VALIDATION_LAYERS.len() as u32
        } else {
            0
        },
        pp_enabled_layer_names: if USE_VALIDATION_LAYERS {
            layer_names.as_ptr()
        } else {
            ptr::null()
        },
        p_next: if USE_VALIDATION_LAYERS {
            &create_debug_create_info() as *const vk::DebugUtilsMessengerCreateInfoEXT
                as *const c_void
        } else {
            ptr::null()
        },
        ..Default::default()
    };

    unsafe { entry.create_instance(&create_info, None).unwrap() }
}

extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = unsafe { CStr::from_ptr((*p_callback_data).p_message) };
    println!("{} {} {:?}", severity, types, message);

    vk::FALSE
}

pub fn create_debug_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    let create_info = vk::DebugUtilsMessengerCreateInfoEXT {
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        pfn_user_callback: Some(vulkan_debug_utils_callback),
        ..Default::default()
    };
    create_info
}

pub fn create_debug_utils(
    entry: &ash::Entry,
    instance: &ash::Instance,
) -> (ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT) {
    let debug_utils = ash::extensions::ext::DebugUtils::new(entry, instance);

    if !USE_VALIDATION_LAYERS {
        return (debug_utils, vk::DebugUtilsMessengerEXT::null());
    }

    let messenger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&create_debug_create_info(), None)
            .unwrap()
    };

    (debug_utils, messenger)
}
