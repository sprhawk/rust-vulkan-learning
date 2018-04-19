
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
impl_vertex!(Vertex, position);


pub mod default_vertex_shader {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
    #version 450
    layout(location = 0) in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "]
    #[allow(dead_code)]
    struct Dummy;
}

pub mod default_fragment_shader {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
    #version 450
    layout(location = 0) out vec4 f_color;

    void main() {
        f_color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "]
    #[allow(dead_code)]
    struct Dummy;
}