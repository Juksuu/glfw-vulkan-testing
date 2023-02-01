use ash::vk;

pub struct Renderer {
    entry: ash::Entry,
    instance: ash::Instance,
}

impl Renderer {
    pub fn new(window: &glfw::Window) -> Self {
        let entry = ash::Entry::linked();

        let app_info = vk::ApplicationInfo {
            api_version: vk::API_VERSION_1_3,
            ..Default::default()
        };

        use raw_window_handle::HasRawDisplayHandle;
        let extensions =
            ash_window::enumerate_required_extensions(window.raw_display_handle()).unwrap();

        let instance_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&instance_info, None).unwrap() };

        Self { entry, instance }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) }
    }
}
