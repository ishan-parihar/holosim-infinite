use egui_wgpu::{Renderer, ScreenDescriptor};
fn test(
    renderer: &mut Renderer,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
) {
    let sd = ScreenDescriptor {
        size_in_pixels: [800, 600],
        pixels_per_point: 1.0,
    };
}
