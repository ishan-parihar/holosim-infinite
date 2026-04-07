//! Typed GPU buffer wrappers with staging support.
//!
//! Provides `GpuBuffer` for generic GPU buffers and `StagingBuffer` for
//! CPU↔GPU data transfer with proper memory mapping.

use wgpu::util::DeviceExt;

/// Generic GPU buffer wrapper.
pub struct GpuBuffer {
    pub buffer: wgpu::Buffer,
    pub size: u64,
    pub usage: wgpu::BufferUsages,
}

impl GpuBuffer {
    /// Create a zero-initialized buffer of the given size.
    pub fn new(
        device: &wgpu::Device,
        size: u64,
        usage: wgpu::BufferUsages,
        label: Option<&str>,
    ) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage,
            mapped_at_creation: false,
        });

        GpuBuffer { buffer, size, usage }
    }

    /// Create a buffer initialized from typed host data.
    pub fn from_slice<T: bytemuck::Pod>(
        device: &wgpu::Device,
        data: &[T],
        usage: wgpu::BufferUsages,
        label: Option<&str>,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(data),
            usage,
        });

        let size = (std::mem::size_of::<T>() * data.len()) as u64;
        GpuBuffer { buffer, size, usage }
    }

    /// Create a binding resource for use in bind groups.
    pub fn as_binding(&self) -> wgpu::BindingResource {
        self.buffer.as_entire_binding()
    }
}

/// Double-buffered staging area for CPU↔GPU transfers.
///
/// Separates upload (CPU→GPU) and download (GPU→CPU) paths to avoid
/// synchronization stalls.
pub struct StagingBuffer {
    upload: GpuBuffer,
    download: GpuBuffer,
}

impl StagingBuffer {
    /// Create staging buffers for the given number of elements.
    pub fn new<T: bytemuck::Pod>(device: &wgpu::Device, count: usize) -> Self {
        let byte_size = (std::mem::size_of::<T>() * count) as u64;

        let upload = GpuBuffer::new(
            device,
            byte_size,
            wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
            Some("staging_upload"),
        );

        let download = GpuBuffer::new(
            device,
            byte_size,
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            Some("staging_download"),
        );

        StagingBuffer { upload, download }
    }

    /// Write typed data to the upload buffer via the queue.
    pub fn write<T: bytemuck::Pod>(&self, queue: &wgpu::Queue, data: &[T]) {
        queue.write_buffer(&self.upload.buffer, 0, bytemuck::cast_slice(data));
    }

    /// Read typed data back from the download buffer.
    ///
    /// The download buffer must have been populated via a copy command
    /// (e.g., `encoder.copy_buffer_to_buffer`) and the command buffer submitted
    /// before calling this method.
    pub async fn read<T: bytemuck::Pod>(&self, device: &wgpu::Device) -> Vec<T> {
        let buffer_slice = self.download.buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        device.poll(wgpu::Maintain::Wait);
        receiver.recv().expect("map_async channel closed").unwrap();

        let data = buffer_slice.get_mapped_range();
        let result = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        self.download.buffer.unmap();
        result
    }

    /// Get the upload buffer for copy operations.
    pub fn upload(&self) -> &GpuBuffer {
        &self.upload
    }

    /// Get the download buffer for copy operations.
    pub fn download(&self) -> &GpuBuffer {
        &self.download
    }
}
