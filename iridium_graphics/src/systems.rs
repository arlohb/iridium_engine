use iridium_ecs::*;

use crate::*;

pub struct Renderer2DSystem {}

impl Renderer2DSystem {
    pub fn run(&mut self, entities: &Entities, _delta_time: f64, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        for [mut transform, mut renderable_2d]
        in entities.query(["Transform", "Renderable2D"]) {
            // Extend the lifetime of renderable_2d for render_pass.set_pipeline.
            // This is safe because it's only used as the pipeline for the duration of this function.
            let renderable_2d = unsafe { std::mem::transmute::<&mut Component, &mut Component>(&mut renderable_2d) };

            let renderable_2d = renderable_2d.component::<Renderable2D>();

            let material = &renderable_2d.material;

            let transform = transform.component::<Transform>();

            let transform_bytes = transform.position.as_bytes::<16>().into_iter()
                .chain(transform.scale.as_bytes::<12>().into_iter())
                .chain(transform.rotation.to_le_bytes().into_iter())
                .collect::<Vec<u8>>();

            queue.write_buffer(
                &material.vertex_data.buffers[0],
                0,
                &transform_bytes,
            );

            render_pass.set_pipeline(&material.material.render_pipeline);
            render_pass.set_bind_group(0, &material.vertex_data.bind_group, &[]);
            render_pass.set_bind_group(1, &material.fragment_data.bind_group, &[]);
            render_pass.set_vertex_buffer(0, renderable_2d.vertex_buffer.slice(..));
            render_pass.set_index_buffer(renderable_2d.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..renderable_2d.index_count, 0, 0..1);
        }
    }
}
