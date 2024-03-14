use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use crate::vertex;
use crate::pipeline;
use crate::texture;
use crate::mesh;

pub struct App<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    pipelines: Vec<pipeline::Pipeline>,
    textures: Vec<texture::Texture>,
    meshes: Vec<mesh::Mesh<'a>>,
    window: Window
}

impl<'a> App<'a> {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe {
            instance.create_surface_unsafe(
                wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap()
            ).unwrap()
        };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|format| format.is_srgb())
            .next()
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: Vec::new()
        };
        surface.configure(&device, &config);

        App {
            surface,
            device,
            queue,
            config,
            size,
            pipelines: Vec::new(),
            textures: Vec::new(),
            meshes: Vec::new(),
            window
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 &&new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update(&mut self) {

    }

    fn render(
        &mut self,
        pipeline_index: usize,
        mesh_index: usize
    ) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.01,
                        b: 0.01,
                        a: 1.0
                    }),
                    store: wgpu::StoreOp::Store
                }
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None
        });
        render_pass.set_pipeline(self.pipelines[pipeline_index].pipeline());

        for i in 0..self.textures.len() {
            render_pass.set_bind_group(i as u32, self.textures[i].bind_group(), &[])
        }

        render_pass.set_vertex_buffer(0, self.meshes[mesh_index].vertex_buffer().slice(..));
        render_pass.set_index_buffer(self.meshes[mesh_index].index_buffer().slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.meshes[mesh_index].indices().len() as u32, 0, 0..1);
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn create_shader(&self, desc: wgpu::ShaderModuleDescriptor) -> wgpu::ShaderModule {
        self.device.create_shader_module(desc)
    }

    fn add_pipelines(&mut self, pipelines: Vec<pipeline::Pipeline>) {
        for pipeline in pipelines {
            self.pipelines.push(pipeline);
        }
    }

    fn add_textures(&mut self, textures: Vec<texture::Texture>) {
        for texture in textures {
            self.textures.push(texture);
        }
    }

    fn add_meshes(&mut self, meshes: Vec<mesh::Mesh<'a>>) {
        for mesh in meshes {
            self.meshes.push(mesh);
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

pub async fn run() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()
        .unwrap();
    let window = WindowBuilder::new()
        .with_title("GFX Sandbox")
        .build(&event_loop)
        .unwrap();

    let mut app = App::new(window).await;

    app.add_textures(
        vec![
            texture::Texture::new(
                &app,
                include_bytes!("../resources/textures/smile.png"),
                wgpu::FilterMode::Nearest,
                wgpu::AddressMode::Repeat,
                wgpu::ShaderStages::FRAGMENT
            )
        ]
    );

    app.add_meshes(
        vec![
            mesh::Mesh::new(
                &app,
                bytemuck::cast_slice(vertex::TRIANGLE_COLOR),
                vertex::VertexType::VertexColor,
                vertex::TRIANGLE_COLOR_INDICES
            ),
            mesh::Mesh::new(
                &app,
                bytemuck::cast_slice(vertex::SQUARE_UV),
                vertex::VertexType::VertexUV,
                vertex::SQUARE_UV_INDICES
            )
        ]
    );

    let shader_interpolate = app.create_shader(wgpu::include_wgsl!("../resources/shaders/interpolate.wgsl"));
    let shader_uv = app.create_shader(wgpu::include_wgsl!("../resources/shaders/uv.wgsl"));
    app.add_pipelines(
        vec![
            pipeline::Pipeline::new(
                &app,
                &shader_interpolate,
                &[],
                vertex::VertexType::VertexColor
            ),
            pipeline::Pipeline::new(
                &app,
                &shader_uv,
                &[app.textures[0].bind_group_layout()],
                vertex::VertexType::VertexUV
            )
        ]
    );

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => app.resize(physical_size),
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                app.update();
                app.render(1, 1);
            },
            _ => ()
        }

        app.window.request_redraw();
    })
}
