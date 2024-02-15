use wgpu::util::DeviceExt;

fn main() {
    let wgpu = create_wgpu();

    let event_loop = create_event_loop();

    let window = create_window(&event_loop);

    // TODO: Ошибка E0515, неясно как вернуть значение из функции
    let surface = wgpu.create_surface(&window).unwrap();

    let adapter = create_adapter(&wgpu, &surface);

    let (device, queue) = create_device_and_queue(&adapter);

    let surface_config = get_surface_config(&surface, &adapter, window.inner_size());

    configure_surface(&surface, &device, &surface_config);

    let color_target_state = create_color_target_state(&surface, &adapter);

    let render_pipeline = create_render_pipeline(&device, color_target_state);

    let mut encoder = create_command_encoder(&device);

    let surface_texture = get_current_texture(&surface);

    let data: [f32; 6] = [
        // X, Y
        -0.8, -0.8, 0.8, -0.8, 0.0, 0.8,
    ];
    let buffer = create_buffer(&device, &data);

    {
        let texture_view = create_view(&surface_texture);

        let mut render_pass = create_render_pass(&mut encoder, &texture_view, &render_pipeline);

        render_pass.set_vertex_buffer(0, buffer.slice(..));

        render_pass.draw(0..3, 0..1);
    }

    let command_buffer = encoder.finish();

    queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&data));

    queue.submit([command_buffer]);

    surface_texture.present();

    std::thread::sleep(core::time::Duration::from_secs(3))
}

fn create_wgpu() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor::default())
}

fn create_event_loop() -> winit::event_loop::EventLoop<()> {
    winit::event_loop::EventLoop::new().unwrap()
}

fn create_window(event_loop: &winit::event_loop::EventLoop<()>) -> winit::window::Window {
    winit::window::Window::new(&event_loop).unwrap()
}

fn create_adapter(wgpu: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    pollster::block_on(wgpu.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(surface),
        ..Default::default()
    }))
    .unwrap()
}

fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    pollster::block_on(adapter.request_device(&Default::default(), Default::default())).unwrap()
}

fn get_surface_config(
    surface: &wgpu::Surface,
    adapter: &wgpu::Adapter,
    size: winit::dpi::PhysicalSize<u32>,
) -> wgpu::SurfaceConfiguration {
    surface
        .get_default_config(adapter, size.width, size.height)
        .unwrap()
}

fn configure_surface(
    surface: &wgpu::Surface<'_>,
    device: &wgpu::Device,
    surface_config: &wgpu::SurfaceConfiguration,
) {
    surface.configure(&device, &surface_config);
}

fn create_color_target_state(
    surface: &wgpu::Surface,
    adapter: &wgpu::Adapter,
) -> wgpu::ColorTargetState {
    let texture_format = surface.get_capabilities(&adapter).formats[0];

    wgpu::ColorTargetState {
        format: texture_format,
        blend: Default::default(),
        write_mask: Default::default(),
    }
}

fn create_render_pipeline(
    device: &wgpu::Device,
    color_target_state: wgpu::ColorTargetState,
) -> wgpu::RenderPipeline {
    let shader_module = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Default::default(),
        primitive: Default::default(),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vertex",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 8, // 2 (vec2f: x, y) * 4 (byte of float 32)
                step_mode: Default::default(),
                attributes: &[wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                }],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fragment",
            targets: &[Some(color_target_state)],
        }),
        layout: Default::default(),
        depth_stencil: Default::default(),
        multisample: Default::default(),
        multiview: Default::default(),
    })
}

fn create_command_encoder(device: &wgpu::Device) -> wgpu::CommandEncoder {
    device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
}

fn get_current_texture(surface: &wgpu::Surface) -> wgpu::SurfaceTexture {
    surface.get_current_texture().unwrap()
}

fn create_view(surface_texture: &wgpu::SurfaceTexture) -> wgpu::TextureView {
    surface_texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default())
}

fn create_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    texture_view: &'a wgpu::TextureView,
    render_pipeline: &'a wgpu::RenderPipeline,
) -> wgpu::RenderPass<'a> {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: Default::default(),
            ops: Default::default(),
        })],
        ..Default::default()
    });
    render_pass.set_pipeline(&render_pipeline);
    render_pass
}

fn create_buffer(device: &wgpu::Device, data: &[f32; 6]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Default::default(),
        contents: bytemuck::cast_slice(data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}
