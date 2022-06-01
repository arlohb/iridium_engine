use iridium_ecs::*;
use iridium_maths::Vec3;

use crate::*;

pub struct Renderer2DSystem {}

impl Renderer2DSystem {
    pub fn run(&mut self, entities: &Entities, _delta_time: f64, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        for [transform, renderable_2d]
        in entities.query(["Transform", "Renderable2D"]) {
            let renderable_2d = unsafe {
                let ptr = &renderable_2d as &Component as *const Component;
                &*ptr
            };

            let material = renderable_2d.get::<MaterialInstance>("material");

            let transform_bytes = transform.get::<Vec3>("position").as_bytes::<16>().into_iter()
                .chain(transform.get::<Vec3>("scale").as_bytes::<16>().into_iter())
                .collect::<Vec<u8>>();

            queue.write_buffer(
                &material.vertex_data.buffers[0],
                0,
                &transform_bytes,
            );

            render_pass.set_pipeline(&material.material.render_pipeline);
            render_pass.set_bind_group(0, &material.vertex_data.bind_group, &[]);
            render_pass.set_vertex_buffer(0, renderable_2d.get::<wgpu::Buffer>("vertex_buffer").slice(..));
            render_pass.set_index_buffer(renderable_2d.get::<wgpu::Buffer>("index_buffer").slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..*renderable_2d.get::<u32>("index_count"), 0, 0..1);
        }
    }
}
