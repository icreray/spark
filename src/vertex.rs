pub use spark_macros::{vertex, Vertex};

use bytemuck::Pod;
use wgpu::VertexBufferLayout;

pub trait Vertex: Pod {
    fn buffer_layout<'l>() -> VertexBufferLayout<'l>;
}

#[cfg(test)]
mod tests {
    use crate::vertex::*;

    #[test]
    pub fn is_vertex_attribute_compiled() {
        #[repr(C)]
        #[vertex]
        struct TestVertex {
            #[attribute(0, Float32x3)]
            position: [f32; 3],
            #[attribute(1, Float32x3)]
            normal: [f32; 3]
        }
    }
}
