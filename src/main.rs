#[macro_use]
extern crate vulkano;

extern crate vulkano_win;
extern crate winit;
extern crate image;

use std::sync::Arc;

#[allow(unused_imports)]
use vulkano_win::VkSurfaceBuild;

#[allow(unused_imports)]
use vulkano::instance::{DeviceExtensions, Features, Instance, InstanceExtensions, Limits,
                        PhysicalDevice, QueueFamily};

use vulkano::device::Device;

use vulkano::format::{ClearValue, Format};

use vulkano::image::{Dimensions, StorageImage};

use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use vulkano::instance::debug::DebugCallback;

use vulkano::sync::GpuFuture;

use image::{ImageBuffer, Rgba};

fn main() {
    println!("Hello, Vulkan!");

    let instance = {
        let app_info = app_info_from_cargo_toml!();
        // println!("Application Info:{:?}", app_info);
        let extensions = vulkano_win::required_extensions();
        Instance::new(Some(&app_info), &extensions, None).expect("failed to create Vulkan instance")
    };

    let _callback = DebugCallback::errors_and_warnings(&instance, |msg| {
        println!("Vulkan Debug: {:?}", msg.description);
    }).ok();

    // print_vk_info(&instance);

    let physical_device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device available");

    let queue_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("No graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical_device,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        ).expect("failed to create device")
    };

    let queue = queues.next().expect("No queues are found");

    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue_family),
    ).unwrap();

    let buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        (0..1024 * 1024 * 4).map(|_| 0u8),
    ).expect("Failed to create buffer");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0]))
        .unwrap()
        .copy_image_to_buffer(image.clone(), buffer.clone()).unwrap()
        .build()
        .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let buffer_content = buffer.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();

    /*
    let mut events_loop = winit::EventsLoop::new();
    let _window = winit::WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, ..} => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
        }
    });
    */
}

#[allow(dead_code)]
fn print_vk_info(instance: &Arc<Instance>) {
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

fn print_layers() {
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
