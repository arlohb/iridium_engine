use iridium_ecs::*;

use crate::*;

pub struct Renderer2DSystem {}

impl Renderer2DSystem {
    pub fn run(&mut self, entities: &Entities, _delta_time: f64, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        for [position, renderable_2d] in entities.query(["Position", "Renderable2D"]) {
            let renderable_2d = unsafe {
                let ptr = &renderable_2d as &Component as *const Component;
                &*ptr
            };

            let material = renderable_2d.get::<MaterialInstance>("material");

            let pos_f32 = [
                *position.get::<f64>("x") as f32,
                *position.get::<f64>("y") as f32,
                // *position.get::<f64>("z") as f32,
            ];

            queue.write_buffer(
                &material.vertex_data.buffers[0],
                0,
                &pos_f32.into_iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>(),
            );

            render_pass.set_pipeline(&material.material.render_pipeline);
            render_pass.set_bind_group(0, &material.vertex_data.bind_group, &[]);
            render_pass.set_vertex_buffer(0, renderable_2d.get::<wgpu::Buffer>("vertex_buffer").slice(..));
            render_pass.set_index_buffer(renderable_2d.get::<wgpu::Buffer>("index_buffer").slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..*renderable_2d.get::<u32>("index_count"), 0, 0..1);
        }
    }
}
