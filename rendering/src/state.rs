use anyhow::{anyhow, Result};
use raw_window_handle::HasRawWindowHandle;
use thiserror::Error;
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};

type PixelSize = (u32, u32);

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    staging_belt: wgpu::util::StagingBelt,
    size: PixelSize,
}

#[repr(u32)]
enum Nord {
    PolarNight0 = 0x2e3440,
    PolarNight1 = 0x3b4252,
    PolarNight2 = 0x434c5e,
    PolarNight3 = 0x4c566a,
    SnowStorm0 = 0xd8dee9,
    SnowStorm1 = 0xe5e9f0,
    SnowStorm2 = 0xeceff4,
    Frost0 = 0x8fbcbb,
    Frost1 = 0x88c0d0,
    Frost2 = 0x81a1c1,
    Frost3 = 0x5e81ac,
    Aurora0 = 0xbf616a,
    Aurora1 = 0xd08770,
    Aurora2 = 0xebcb8b,
    Aurora3 = 0xa3be8c,
    Aurora4 = 0xb48ead,
}

impl Nord {
    fn parts(self) -> [u8; 3] {
        let n = self as u32;
        let r = (n >> 16) as u8;
        let g = (n >> 8) as u8;
        let b = n as u8;
        [r, g, b]
    }

    pub fn into_array(self) -> [f32; 4] {
        let parts = self.parts();
        [
            (parts[0] as f32 / 255.0).powf(2.2),
            (parts[1] as f32 / 255.0).powf(2.2),
            (parts[2] as f32 / 255.0).powf(2.2),
            1.0,
        ]
    }

    pub fn into_color(self) -> wgpu::Color {
        let parts = self.parts();
        wgpu::Color {
            r: (parts[0] as f64 / 255.0).powf(2.2),
            g: (parts[1] as f64 / 255.0).powf(2.2),
            b: (parts[2] as f64 / 255.0).powf(2.2),
            a: 1.0,
        }
    }
}

#[derive(Error, Debug)]
enum StateCreationError {
    #[error("Request adapter failed")]
    RequestAdapterFailed,
}

impl State {
    pub async fn new_from_window<W>(size: PixelSize, window: &W) -> Result<Self>
    where
        W: HasRawWindowHandle,
    {
        // Use a backend with first-tier support
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // Gets a destination for rendered images
        let surface = unsafe { instance.create_surface(window) };

        // The physical compute device, e.g. a GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(StateCreationError::RequestAdapterFailed)?;

        // Device is the connection the the adapter
        // Queue executes command buffers
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    // Todo: Would be nice to hike up features and limits once
                    // I have a better GPU, especially to use push constants
                    features: Default::default(),
                    limits: Default::default(),
                    shader_validation: true,
                },
                // Don't need call tracing
                None,
            )
            .await?;

        // Used by wgpu_glyph to upload data. Works similar to queue.write_buffers
        // but does so more efficiently.
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let sc_desc = wgpu::SwapChainDescriptor {
            // As opposed to an intermediate texture, e.g. a shader input
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.0,
            height: size.1,
            // Vsync and refresh rate limited to display framerate
            present_mode: wgpu::PresentMode::Fifo,
        };

        // Multiple textures for presentation to prevent screen tear
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Ok(Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            staging_belt,
            size,
        })
    }

    pub fn resize(&mut self, new_size: PixelSize) {
        self.size = new_size;
        self.sc_desc.width = new_size.0;
        self.sc_desc.height = new_size.1;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn update(&mut self) {
        // Todo
    }

    pub fn render(&mut self) -> Result<()> {
        // Get a texture to draw on
        let mut frame = self.swap_chain.get_current_frame()?.output;

        // Encodes render passes to create a command buffer for submission
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.clear_screen_pass(&mut frame, &mut encoder);

        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("assets/CascadiaMono.ttf"))?;
        let mut glyph_brush =
            GlyphBrushBuilder::using_font(font).build(&self.device, self.sc_desc.format);

        glyph_brush.queue(Section {
            screen_position: (10.0, 10.0),
            bounds: (90.0, 90.0),
            text: vec![Text::new("Hello wgpu_glyph")],
            ..Section::default()
        });

        glyph_brush.queue(Section {
            screen_position: (10.0, 40.0),
            text: vec![Text::new("I appear below")
                .with_color(Nord::SnowStorm2.into_array())
                .with_scale(40.0)],
            ..Section::default()
        });

        glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &frame.view,
                self.size.0,
                self.size.1,
            )
            .map_err(|e| anyhow!("{}", e))?;

        self.staging_belt.finish();
        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }

    fn clear_screen_pass(
        &mut self,
        frame: &mut wgpu::SwapChainTexture,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        // Render pass added on drop
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(Nord::PolarNight0.into_color()),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
    }
}
