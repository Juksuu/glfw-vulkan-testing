use vulkan_library::{instance::Instance, VulkanLibrary};

pub struct Renderer {
    library: VulkanLibrary,
    instance: Instance,
}

impl Renderer {
    pub fn new(window: &crate::Window) -> Self {
        let debug_info = true;

        let mut library = VulkanLibrary::new(debug_info);
        let instance = Instance::new(&mut library, &window.glfw_window);

        let p_devices = instance
            .enumerate_physical_devices()
            .expect("Could not enumerate devices");

        Self { library, instance }
    }
}
