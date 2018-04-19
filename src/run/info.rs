use ::vulkano;
use std::sync::Arc;
use vulkano::instance::{DeviceExtensions, Instance, InstanceExtensions,
                        PhysicalDevice};
                        
#[allow(dead_code)]
pub fn print_vk_info(instance: &Arc<Instance>) {
    print_instance_extensions();
    print_layers();

    for device in PhysicalDevice::enumerate(instance) {
        print_vk_physical_device(&device);
    }
}

fn print_instance_extensions() {
    let exts = InstanceExtensions::supported_by_core().expect("No instance extensions");
    println!("Instance extensions:");
    print!("khr_surface:{} khr_display:{} khr_xlib_surface: {} khr_xcb_surface: {} \
    khr_wayland_surface: {} khr_mir_surface: {} khr_android_surface: {} khr_win32_surface: {} \
    ext_debug_report: {} mvk_ios_surface: {} mvk_macos_surface: {} mvk_moltenvk: {} nn_vi_surface: {} \
    ext_swapchain_colorspace: {} khr_get_phyiscal_device_properties2: {}",
    exts.khr_surface, exts.khr_display, exts.khr_xlib_surface, exts.khr_xcb_surface,
    exts.khr_wayland_surface, exts.khr_mir_surface, exts.khr_android_surface, exts.khr_win32_surface,
    exts.ext_debug_report, exts.mvk_ios_surface, exts.mvk_macos_surface, exts.mvk_moltenvk, exts.nn_vi_surface,
    exts.ext_swapchain_colorspace, exts.khr_get_physical_device_properties2);
    println!("");
}

fn print_physical_device_extensions(device: &PhysicalDevice) {
    let exts = DeviceExtensions::supported_by_device(*device);
    println!("Device extensions:");
    print!(
        "khr_swapchain: {} khr_display_swapchain: {} khr_sampler_mirror_clamp_to_edge: {} \
         khr_maintenance1: {} khr_get_memory_requirements: {} khr_dedicated_allocation: {} \
         khr_incremental_present: {} ext_debug_marker: {}",
        exts.khr_swapchain,
        exts.khr_display_swapchain,
        exts.khr_sampler_mirror_clamp_to_edge,
        exts.khr_maintenance1,
        exts.khr_get_memory_requirements2,
        exts.khr_dedicated_allocation,
        exts.khr_incremental_present,
        exts.ext_debug_marker
    );
}

pub fn print_layers() {
    if let Ok(layers_list) = vulkano::instance::layers_list() {
        println!("Available layers:");
        for layer in layers_list {
            println!("{} : {}", layer.name(), layer.description());
        }
    }
    println!("");
}

fn print_vk_physical_device(device: &PhysicalDevice) {
    print!("Device Info: ");
    print!("Name:{} ", device.name());
    print!("Type:{:?}", device.ty());
    print!("\n");
    print!(
        "Api: {:?} Driver: {}",
        device.api_version(),
        device.driver_version()
    );
    print!("\n");
    println!("Supported Features:");
    println!("{:?}", device.supported_features());
    println!("Queue families:");
    for queue_family in device.queue_families() {
        println!(
            "queue {}: count: {} graphics:{} compute:{} transfers:{} sparse_bind:{}",
            queue_family.id(),
            queue_family.queues_count(),
            queue_family.supports_graphics(),
            queue_family.supports_compute(),
            queue_family.supports_transfers(),
            queue_family.supports_sparse_binding()
        );
    }

    for mem_type in device.memory_types() {
        println!("memtype {}: local:{}, host_visible: {}, host_coherent: {}, host_cached: {}, lazily_allocated: {}", 
        mem_type.id(),
        mem_type.is_device_local(),
        mem_type.is_host_visible(),
        mem_type.is_host_coherent(),
        mem_type.is_host_cached(),
        mem_type.is_lazily_allocated()
        );
    }

    for mem_heap in device.memory_heaps() {
        println!(
            "memheap {}: size: {}, local: {}",
            mem_heap.id(),
            mem_heap.size(),
            mem_heap.is_device_local()
        );
    }

    let lim = device.limits();
    println!("Limits:");
    println!("max_image_dimension_1d: {}", lim.max_image_dimension_1d());
    println!("max_image_dimension_2d: {}", lim.max_image_dimension_2d());
    println!("max_image_dimension_3d: {}", lim.max_image_dimension_3d());
    println!(
        "max_image_dimension_cube: {}",
        lim.max_image_dimension_cube()
    );

    print_physical_device_extensions(device);
}
