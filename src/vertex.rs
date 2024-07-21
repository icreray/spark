pub use spark_macros::{vertex, unsafe_vertex, Vert};

use bytemuck::Pod;
use wgpu::VertexBufferLayout;

pub trait Vert: Pod {
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

    #[test]
    pub fn is_unsafe_vertex_attribute_compiled() {
        #[repr(C)]
        #[unsafe_vertex]
        struct TestVertex {
            #[attribute(0, Float32x3)]
            position: [f32; 3],
            #[attribute(1, Float32x3)]
            normal: [f32; 3]
        }
    }
}
