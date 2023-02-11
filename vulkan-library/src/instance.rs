use std::ffi::{CStr, CString};

use ash::vk;
use raw_window_handle::HasRawDisplayHandle;

use crate::{
    debug::{self, DebugUtils},
    VulkanLibrary,
};

fn validate_requested_layers(library: &VulkanLibrary) -> Vec<&str> {
    let mut layers: Vec<&str> = Vec::new();

    if library.debug {
        layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
    }

    let layer_properties = library.entry.enumerate_instance_layer_properties().unwrap();

    let layer_properties_str: Vec<&str> = layer_properties
        .iter()
        .map(|l| unsafe { CStr::from_ptr(l.layer_name.as_ptr()).to_str().unwrap() })
        .collect();

    let layers_supported = layers
        .iter()
        .all(|layer| layer_properties_str.contains(layer));

    if !layers_supported {
        panic!(
            "Not all requested validation layers are supported \n requested: {:?} \n available: {:?}",
            layers, layer_properties_str
        );
    }

    layers
}

pub struct Instance {
    ash_instance: ash::Instance,
}

impl Instance {
    pub fn new(library: &mut VulkanLibrary, window: &impl HasRawDisplayHandle) -> Self {
        let mut extensions: Vec<*const i8> =
            ash_window::enumerate_required_extensions(window.raw_display_handle())
                .unwrap()
                .to_vec();

        let layer_names_cstring: Vec<CString> = validate_requested_layers(library)
            .iter()
            .map(|l| CString::new(l.to_owned()).unwrap())
            .collect();
        let layers: Vec<*const i8> = layer_names_cstring.iter().map(|l| l.as_ptr()).collect();

        if library.debug {
            extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());
        }

        let instance_info_builder = vk::InstanceCreateInfo::builder()
            .application_info(&library.app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);

        let instance_info = if library.debug {
            instance_info_builder
                .push_next(&mut debug::create_debug_info())
                .build()
        } else {
            instance_info_builder.build()
        };

        let ash_instance = unsafe { library.entry.create_instance(&instance_info, None).unwrap() };

        if library.debug {
            library.debug_utils = Some(DebugUtils::new(&library, &ash_instance))
        }

        Self { ash_instance }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.ash_instance.destroy_instance(None) }
    }
}
