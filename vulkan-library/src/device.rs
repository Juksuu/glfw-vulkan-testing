use ash::vk;

#[derive(Debug)]
pub struct PhysicalDevice {
    device: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    queue_families: Vec<vk::QueueFamilyProperties>,
}

impl PhysicalDevice {
    pub(crate) fn new(instance: &ash::Instance, device: vk::PhysicalDevice) -> Self {
        let properties = unsafe { instance.get_physical_device_properties(device) };
        let features = unsafe { instance.get_physical_device_features(device) };
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(device) };

        Self {
            device,
            properties,
            features,
            queue_families,
        }
    }
}
