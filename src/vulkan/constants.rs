#[cfg(not(debug_assertions))]
pub const USE_VALIDATION_LAYERS: bool = false;

#[cfg(debug_assertions)]
pub const USE_VALIDATION_LAYERS: bool = true;

pub const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];
