mod info;
mod shaders;

use std::sync::Arc;

#[allow(unused_imports)]
use vulkano_win::VkSurfaceBuild;

#[allow(unused_imports)]
use vulkano::instance::{DeviceExtensions, Features, Instance, InstanceExtensions, Limits,
                        PhysicalDevice, QueueFamily};

use vulkano::device::{Device, Queue};

#[allow(unused_imports)]
use vulkano::format::{ClearValue, Format};

// use vulkano::image::{Dimensions, StorageImage};

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::framebuffer::{Framebuffer, Subpass};

use vulkano::instance::debug::DebugCallback;

use vulkano::sync::{now, GpuFuture};

use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;

use vulkano::swapchain;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};

use vulkano_win;
use winit;

use self::shaders::Vertex;

/*
use ::image;
#[allow(unused_imports)]
use image::{ImageBuffer, Rgba};
*/

pub struct VulkanStruct {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

fn create_vulkan_struct() -> VulkanStruct {
    let instance = {
        let app_info = app_info_from_cargo_toml!();
        // println!("Application Info:{:?}", app_info);
        let extensions = vulkano_win::required_extensions();
        Instance::new(Some(&app_info), &extensions, None).expect("failed to create Vulkan instance")
    };

    let _callback = DebugCallback::errors_and_warnings(&instance, |msg| {
        println!("Vulkan Debug: {:?}", msg.description);
    }).ok();

    info::print_vk_info(&instance);

    let physical_device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device available");

    let queue_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("No graphical queue family");

    let (device, mut queues) = {
        let ext = DeviceExtensions {
            khr_swapchain: true,
            .. DeviceExtensions::none()
        };

        Device::new(
            physical_device,
            &Features::none(),
            &ext,
            [(queue_family, 0.5)].iter().cloned(),
        ).expect("failed to create device")
    };

    let queue = queues.next().expect("No queues are found");
    

    VulkanStruct {
        device: device,
        queue: queue,
    }
}

pub fn run() {
    
    let vulkan_obj = create_vulkan_struct();

    let vertex_shader = shaders::default_vertex_shader::Shader::load(vulkan_obj.device.clone())
        .expect("Failed to create vertex shader module");
    let fragment_shader = shaders::default_fragment_shader::Shader::load(vulkan_obj.device.clone())
        .expect("Failed to create fragment shader module");


    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .build_vk_surface(&events_loop, vulkan_obj.device.physical_device().instance().clone())
        .unwrap();

    // if do not call is_supported, validation layer will report warnings
    let _r = window.is_supported(vulkan_obj.queue.family()).unwrap();

    let _win = window.window();
    let caps = window
        .capabilities(vulkan_obj.device.physical_device())
        .expect("failed to get surface capabalities");

    let dim = caps.current_extent.unwrap_or([1280, 1024]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    let (swap_chain, images) = Swapchain::new(
        vulkan_obj.device.clone(),
        window.clone(),
        caps.min_image_count,
        format,
        dim,
        1,
        caps.supported_usage_flags,
        &vulkan_obj.queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        true,
        None,
    ).expect("Failed to create swapchain");

    /*
    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue_family),
    ).unwrap();
    */
    let (image_index, swapchain_acquire_future) =
        swapchain::acquire_next_image(swap_chain.clone(), None).unwrap();
    let image = images[image_index].clone();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };

    
    let render_pass = Arc::new(
        single_pass_renderpass!(vulkan_obj.device.clone(),
    attachments: {
        color: {
            load: Clear,
            store: Store,
            format: Format::R8G8B8A8Unorm,
            samples: 1,
        }
    },
    pass: {
        color: [color],
        depth_stencil: {}
    }).unwrap(),
    );

    let framebuffer = Arc::new(
        Framebuffer::start(render_pass.clone())
            .add(image.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vertex_shader.main_entry_point(), ())
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fragment_shader.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(vulkan_obj.device.clone())
            .unwrap(),
    );

    let dynamic_state = DynamicState {
        viewports: Some(vec![
            Viewport {
                origin: [0.0, 0.0],
                dimensions: [1024.0, 1024.0],
                depth_range: 0.0..1.0,
            },
        ]),
        ..DynamicState::none()
    };

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        vulkan_obj.device.clone(),
        BufferUsage::all(),
        vec![vertex1, vertex2, vertex3].into_iter(),
    ).unwrap();

    let buffer = CpuAccessibleBuffer::from_iter(
        vulkan_obj.device.clone(),
        BufferUsage::all(),
        (0..1024 * 1024 * 4).map(|_| 0u8),
    ).expect("Failed to create buffer");

    let command_buffer =
        AutoCommandBufferBuilder::primary_one_time_submit(vulkan_obj.device.clone(), vulkan_obj.queue.family())
            .unwrap()
            .begin_render_pass(
                framebuffer.clone(),
                false,
                vec![[0.0, 0.0, 1.0, 1.0].into()],
            )
            .unwrap()
            .draw(
                pipeline.clone(),
                dynamic_state,
                vertex_buffer.clone(),
                (),
                (),
            )
            .unwrap()
            .end_render_pass()
            .unwrap()
            .copy_image_to_buffer(image.clone(), buffer.clone())
            .unwrap()
            .build()
            .unwrap();

    /*
    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();
*/
    let mut previous_frame_end = Box::new(now(vulkan_obj.device.clone())) as Box<GpuFuture>;

    previous_frame_end.cleanup_finished();
    /*
    let buffer_content = buffer.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();
    */

    let future = previous_frame_end
        .join(swapchain_acquire_future)
        .then_execute(vulkan_obj.queue.clone(), command_buffer)
        .unwrap()
        .then_swapchain_present(vulkan_obj.queue.clone(), swap_chain.clone(), image_index)
        .then_signal_fence_and_flush();

    let _future = future.unwrap();
    // swapchain::present(swap_chain, finished, queue.clone(), image_index);

    events_loop.run_forever(|event| match event {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::Closed,
            ..
        } => winit::ControlFlow::Break,
        _ => winit::ControlFlow::Continue,
    });
}
