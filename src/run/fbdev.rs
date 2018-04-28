use super::VulkanStruct;

use std::sync::Arc;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::instance::InstanceExtensions;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};

use vulkano::swapchain::display::Display;

use vulkano::instance::PhysicalDevice;

pub fn required_extensions() -> InstanceExtensions {
    let extensions = InstanceExtensions {
        khr_display: true,
        ..InstanceExtensions::none()
    };
    extensions
}

/*
pub fn create_swapchain(
    vulkan_obj: Arc<VulkanStruct>,
) -> Result<
    (
        Arc<Swapchain<Display>>,
        Vec<Arc<SwapchainImage<Display>>>
    ),
    SwapchainCreationError
> {
    )
}
*/

pub fn run_loop() {
    loop {}
}

pub fn print_all_displays(physical_device:PhysicalDevice) {
    println!("Displays:");
    for display in Display::enumerate(physical_device) {
        let dim = display.physical_dimensions();
        let resolution = display.physical_resolution();
        println!(
            "name: {} dimension({} x {}) resolution({} x {})",
            display.name(),
            dim[0],
            dim[1],
            resolution[0],
            resolution[1]
        );
        println!("modes:");

        for mode in display.display_modes() {
            let region = mode.visible_region();
            let rate = mode.refresh_rate();
            println!(
                "region({} x {}) refresh rate: {}",
                region[0], region[1], rate
            );
        }
    }
}
