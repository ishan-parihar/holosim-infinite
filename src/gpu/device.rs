//! GPU device initialization and context management.
//!
//! Provides `GpuContext` which encapsulates a wgpu device, queue, and adapter.
//! Returns `None` gracefully when no GPU is available, enabling CPU fallback.

/// GPU context holding the device, queue, and adapter.
///
/// This is the primary handle for all GPU operations in HoloSim.
/// Creation is async and returns `None` when no compatible GPU is found,
/// allowing the simulation to fall back to CPU computation.
pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

impl GpuContext {
    /// Initialize a GPU context using all available backends.
    ///
    /// # Process
    /// 1. Create a `wgpu::Instance` with all backends enabled
    /// 2. Request an adapter with default settings
    /// 3. Request device with default limits and features
    ///
    /// # Returns
    /// - `Some(GpuContext)` when a GPU is successfully initialized
    /// - `None` if no compatible GPU is available (use CPU fallback)
    ///
    /// # Example
    /// ```ignore
    /// let gpu = GpuContext::new().await;
    /// match gpu {
    ///     Some(ctx) => println!("GPU initialized"),
    ///     None => println!("No GPU available, using CPU fallback"),
    /// }
    /// ```
    pub async fn new() -> Option<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("HoloSim GPU Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .ok()?;

        Some(GpuContext {
            device,
            queue,
            adapter,
        })
    }

    /// Get the adapter's information for diagnostics.
    pub fn adapter_info(&self) -> wgpu::AdapterInfo {
        self.adapter.get_info()
    }

    /// Get the device limits for buffer size calculations.
    pub fn limits(&self) -> wgpu::Limits {
        self.device.limits()
    }
}
