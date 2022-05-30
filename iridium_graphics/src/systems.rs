use iridium_ecs::*;

pub struct Renderer2DSystem {}

impl Renderer2DSystem {
    pub fn run(&mut self, entities: &Entities, _delta_time: f64, render_pass: &mut wgpu::RenderPass) {
        for [renderable_2d] in entities.query(["Renderable2D"]) {
            let renderable_2d = unsafe {
                let ptr = &renderable_2d as &Component as *const Component;
                &*ptr
            };
            render_pass.set_pipeline(renderable_2d.get::<wgpu::RenderPipeline>("render_pipeline"));
            render_pass.set_vertex_buffer(0, renderable_2d.get::<wgpu::Buffer>("vertex_buffer").slice(..));
            render_pass.set_index_buffer(renderable_2d.get::<wgpu::Buffer>("index_buffer").slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..*renderable_2d.get::<u32>("index_count"), 0, 0..1);
        }
    }
}
