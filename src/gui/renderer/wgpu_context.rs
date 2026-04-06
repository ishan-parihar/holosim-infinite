//! WGPU Context - Actual GPU initialization and management
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md:
//! "Implement actual WGPU initialization with instance, adapter, device, queue, and surface"

use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

/// WGPU Context - Manages all GPU resources
///
/// This replaces the placeholder WgpuRenderer with actual GPU objects.
pub struct WgpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Option<Surface<'static>>,
    pub surface_config: SurfaceConfiguration,
}

impl std::fmt::Debug for WgpuContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WgpuContext")
            .field("surface_config", &self.surface_config)
            .field("has_surface", &self.surface.is_some())
            .finish()
    }
}

impl WgpuContext {
    /// Initialize WGPU with a window
    pub async fn new(window: std::sync::Arc<Window>) -> Result<Self, String> {
        let backend_order = [
            (wgpu::Backends::VULKAN, "Vulkan"),
            (wgpu::Backends::GL, "OpenGL"),
            (wgpu::Backends::BROWSER_WEBGPU, "WebGPU"),
            (wgpu::Backends::DX12, "DX12"),
            (wgpu::Backends::METAL, "Metal"),
            (wgpu::Backends::all(), "All backends"),
        ];

        let mut last_error = String::from("No backends attempted");

        for (backends, name) in backend_order {
            println!("  Trying WGPU backend: {}...", name);
            match Self::try_init(window.clone(), backends, false).await {
                Ok(ctx) => return Ok(ctx),
                Err(e) => {
                    last_error = e;
                    eprintln!("    {} failed: {}", name, last_error);
                }
            }

            if backends != wgpu::Backends::all() {
                match Self::try_init(window.clone(), backends, true).await {
                    Ok(ctx) => return Ok(ctx),
                    Err(e) => {
                        eprintln!("    {} (fallback) failed: {}", name, e);
                    }
                }
            }
        }

        Err(format!("All WGPU backends failed. Last error: {}", last_error))
    }

    async fn try_init(
        window: std::sync::Arc<Window>,
        backends: wgpu::Backends,
        force_fallback: bool,
    ) -> Result<Self, String> {
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| format!("Failed to create surface: {}", e))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: if force_fallback {
                    wgpu::PowerPreference::None
                } else {
                    wgpu::PowerPreference::HighPerformance
                },
                compatible_surface: Some(&surface),
                force_fallback_adapter: force_fallback,
            })
            .await
            .ok_or("No adapter found")?;

        let adapter_info = adapter.get_info();
        if force_fallback {
            println!("  Using fallback adapter: {} (type: {:?})", adapter_info.name, adapter_info.device_type);
        } else {
            println!("  Using adapter: {} (type: {:?})", adapter_info.name, adapter_info.device_type);
        }

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WGPU Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to create device: {}", e))?;

        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .first()
            .ok_or("No surface format available")?;

        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![*format],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        Ok(WgpuContext {
            instance,
            adapter,
            device,
            queue,
            surface: Some(surface),
            surface_config,
        })
    }

    /// Resize the surface
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        if let Some(surface) = &self.surface {
            surface.configure(&self.device, &self.surface_config);
        }
    }
}
