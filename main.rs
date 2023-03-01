use wgpu::util::DeviceExt;

const VERTICES: &[f32] = &[
    // Front face
    -0.5, -0.5,  0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    // Back face
    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
];

const INDICES: &[u16] = &[
    // Front face
    0, 1, 1, 2, 2, 3, 3, 0,
    // Back face
    4, 5, 5, 6, 6, 7, 7, 4,
    // Connections between front and back face
    0, 4, 1, 5, 2, 6, 3, 7,
];

struct Camera {
    position: nalgebra::Point3<f32>,
    direction: nalgebra::Vector3<f32>,
}

impl Camera {
    fn view_matrix(&self) -> nalgebra::Matrix4<f32> {
        nalgebra::Matrix4::look_at_rh(&self.position, &(self.position + self.direction), &nalgebra::Vector3::y())
    }
}

#[derive(Debug)]
enum Error {
    Wgpu(wgpu::Error),
    Io(std::io::Error),
}

impl From<wgpu::Error> for Error {
    fn from(err: wgpu::Error) -> Self {
        Error::Wgpu(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        compatible_surface: None,
    }).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
    }).await;

    let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertices Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsage::VERTEX,
    });
    let indices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Indices Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsage::INDEX,
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let render_pipeline = device.create_render_pipeline

    &wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &device.create_shader_module(wgpu::include_spirv!("shader.vert.spv")),
            entry_point: "main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float3,
                    shader_location: 0,
                }],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &device.create_shader_module(wgpu::include_spirv!("shader.frag.spv")),
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            strip_index_format: None,
            clamp_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    });

    let mut camera = Camera {
        position: nalgebra::Point3::new(0.0, 0.0, -2.0),
        direction: nalgebra::Vector3::new(0.0, 0.0, 1.0),
    };

    let mut pressed_keys = std::collections::HashSet::new();

    let window = winit::window::Window::new(&winit::window::WindowBuilder::new()
        .with_title("Cube")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0)));
    let mut size = window.inner_size();

    let surface = wgpu::Surface::create(&window);
    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        },
    );

    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);

    let mut last_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::Resized(new_size), .. } => {
                size = new_size;
                swap_chain = device.create_swap_chain(
                    &surface,
                    &wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: size.width,
                        height: size.height,
                        present_mode: w

     &wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &device.create_shader_module(wgpu::include_spirv!("shader.vert.spv")),
            entry_point: "main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float3,
                    shader_location: 0,
                }],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &device.create_shader_module(wgpu::include_spirv!("shader.frag.spv")),
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            strip_index_format: None,
            clamp_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    });

    let mut camera = Camera {
        position: nalgebra::Point3::new(0.0, 0.0, -2.0),
        direction: nalgebra::Vector3::new(0.0, 0.0, 1.0),
    };

    let mut pressed_keys = std::collections::HashSet::new();

    let window = winit::window::Window::new(&winit::window::WindowBuilder::new()
        .with_title("Cube")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0)));
    let mut size = window.inner_size();

    let surface = wgpu::Surface::create(&window);
    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        },
    );

    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);

    let mut last_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::Resized(new_size), .. } => {
                size = new_size;
                swap_chain = device.create_swap_chain(
                    &surface,
                    &wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: size.width,
                        height: size.height,
                        present_mode: w

            }
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                let frame = swap_chain.get_current_frame().unwrap().output;
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                // Update camera position and direction based on keyboard input
                let delta_time = last_frame_time.elapsed().as_secs_f32();
                let speed = 2.0;
                let mut camera_move_direction = nalgebra::Vector3::new(0.0, 0.0, 0.0);
                if pressed_keys.contains(&winit::event::VirtualKeyCode::W) {
                    camera_move_direction += camera.direction;
                }
                if pressed_keys.contains(&winit::event::VirtualKeyCode::S) {
                    camera_move_direction -= camera.direction;
                }
                if pressed_keys.contains(&winit::event::VirtualKeyCode::A) {
                    camera_move_direction += nalgebra::Vector3::new(-camera.direction.z, 0.0, camera.direction.x);
                }
                if pressed_keys.contains(&winit::event::VirtualKeyCode::D) {
                    camera_move_direction += nalgebra::Vector3::new(camera.direction.z, 0.0, -camera.direction.x);
                }
                if camera_move_direction.magnitude_squared() > 0.0 {
                    camera_move_direction.normalize_mut();
                    camera.position += speed * delta_time * camera_move_direction;
                }
                let mouse_delta = window.cursor_position().unwrap().to_physical(window.scale_factor()) - winit::dpi::PhysicalPosition::new(size.width as f64 / 2.0, size.height as f64 / 2.0);
                window.set_cursor_position(winit::dpi::PhysicalPosition::new(size.width as f64 / 2.0, size.height as f64 / 2.0)).unwrap();
                let sensitivity = 0.002;
                let yaw = nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Vector3::new(0.0, 1.0, 0.0), -sensitivity * mouse_delta.x as f32);
                let pitch = nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Vector3::new(1.0, 0.0, 0.0), -sensitivity * mouse_delta.y as f32);
                camera.direction = pitch * yaw * camera.direction;

                // Update uniforms buffer
                let uniforms = Uniforms {
                    view: camera.get_view_matrix().into(),
                    projection: camera.get_projection_matrix(size.width as f32 / size.height as f32).into(),
                };
                queue.write_buffer(&uniforms_buffer, 0, bytemuck::bytes_of(&uniforms));

                // Draw cube
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_bind_group(0, &uniforms_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertices.slice(..));
                    render_pass.draw(0..8, 0..1);
                }

                queue.submit
