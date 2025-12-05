use cgmath::*;
use wgpu::*;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::vertex::Vertex;

#[allow(dead_code)]
pub struct CubeRenderer {
    vertices: Vec<Vertex>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    transform: Matrix4<f32>,
}

impl CubeRenderer {
    pub fn new(device: &Device, position: Vector3<f32>, size: Vector3<f32>) -> Self {
        let (vertices, indices) = Self::create_cube_geometry(position, size);
        
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });
        
        let transform = Matrix4::from_translation(position);
        
        Self {
            vertices,
            vertex_buffer,
            index_buffer,
            transform,
        }
    }
    
    fn create_cube_geometry(_position: Vector3<f32>, size: Vector3<f32>) -> (Vec<Vertex>, Vec<u16>) {
        let l_length = size.x / 2.0;
        let l_height = size.y / 2.0;
        let l_width = size.z / 2.0;
        
        let color = [1.0, 1.0, 1.0, 1.0];
        
        let vertices = vec![
            // Front face
            Vertex { position: [l_length, -l_height, -l_width], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [-l_length, -l_height, -l_width], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [-l_length, l_height, -l_width], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [l_length, l_height, -l_width], normal: [0.0, 0.0, -1.0], color },
            
            // Back face
            Vertex { position: [-l_length, -l_height, l_width], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [l_length, -l_height, l_width], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [l_length, l_height, l_width], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [-l_length, l_height, l_width], normal: [0.0, 0.0, 1.0], color },
            
            // Right face
            Vertex { position: [l_length, -l_height, l_width], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [l_length, -l_height, -l_width], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [l_length, l_height, -l_width], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [l_length, l_height, l_width], normal: [1.0, 0.0, 0.0], color },
            
            // Left face
            Vertex { position: [-l_length, -l_height, -l_width], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-l_length, -l_height, l_width], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-l_length, l_height, l_width], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-l_length, l_height, -l_width], normal: [-1.0, 0.0, 0.0], color },
            
            // Bottom face
            Vertex { position: [-l_length, -l_height, -l_width], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [l_length, -l_height, -l_width], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [l_length, -l_height, l_width], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [-l_length, -l_height, l_width], normal: [0.0, -1.0, 0.0], color },
            
            // Top face
            Vertex { position: [l_length, l_height, -l_width], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [-l_length, l_height, -l_width], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [-l_length, l_height, l_width], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [l_length, l_height, l_width], normal: [0.0, 1.0, 0.0], color },
        ];
        
        // Indices for rendering as triangles (2 triangles per quad face)
        let indices = vec![
            // Front face
            0, 1, 2, 2, 3, 0,
            // Back face
            4, 5, 6, 6, 7, 4,
            // Right face
            8, 9, 10, 10, 11, 8,
            // Left face
            12, 13, 14, 14, 15, 12,
            // Bottom face
            16, 17, 18, 18, 19, 16,
            // Top face
            20, 21, 22, 22, 23, 20,
        ];
        
        (vertices, indices)
    }
    
    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..36, 0, 0..1);
    }
}
