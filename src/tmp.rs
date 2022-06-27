use anyhow::Result;
use thiserror::Error;
use winit::window::Window;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No adapter matches requested options")]
    AdapterNotFound,
    #[error("No device matches descriptor")]
    DeviceNotFound(wgpu::RequestDeviceError),
}

pub struct FrameBuffer {}

impl FrameBuffer {
    pub fn new(window: &Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()));
        let adapter = adapter.ok_or(Error::AdapterNotFound)?;
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                limits: adapter.limits(),
                ..Default::default()
            },
            None,
        ))
        .map_err(Error::DeviceNotFound)?;

        let size = wgpu::Extent3d {
            width: window.inner_size().width,
            height: window.inner_size().height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("framebuffer_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    }
}

use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello World")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_min_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();
    let mut input = WinitInputHelper::new();
}
