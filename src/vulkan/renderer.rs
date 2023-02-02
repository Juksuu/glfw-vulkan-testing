use super::utils;

pub struct Renderer {
    entry: ash::Entry,
    instance: ash::Instance,
    debug_utils: ash::extensions::ext::DebugUtils,
    debug_messenger: ash::vk::DebugUtilsMessengerEXT,
}

impl Renderer {
    pub fn new(window: &crate::Window) -> Self {
        let entry = ash::Entry::linked();
        let instance = utils::create_instance(&entry, window);

        let (debug_utils, debug_messenger) = utils::create_debug_utils(&entry, &instance);

        Self {
            entry,
            instance,
            debug_utils,
            debug_messenger,
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils
                .destroy_debug_utils_messenger(self.debug_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}
