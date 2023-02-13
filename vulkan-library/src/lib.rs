pub mod debug;
pub mod device;
pub mod instance;

use ash::vk;
use debug::DebugUtils;

pub struct VulkanLibrary {
    pub(crate) entry: ash::Entry,
    pub(crate) app_info: vk::ApplicationInfo,

    pub(crate) debug: bool,
    pub(crate) debug_utils: Option<DebugUtils>,
}

impl VulkanLibrary {
    pub fn new(debug: bool) -> Self {
        let app_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_3)
            .build();

        VulkanLibrary {
            entry: ash::Entry::linked(),
            app_info,
            debug,
            debug_utils: None,
        }
    }
}
