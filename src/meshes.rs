use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use crate::cube::CameraConfig;
use std::fs::read_to_string;
use std::ffi::CString;
use std::{ptr, str};


/// compiles a gl shader
pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        // Create GLSL shaders
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!("{}", str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}


/// links a gl shader program
pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        gl::DetachShader(program, fs);
        gl::DetachShader(program, vs);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);

        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!("{}", str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}



#[allow(dead_code)]
#[derive(Clone, Copy)]
/// vertex data array wrapper
pub enum VertexData {
    VD3([GLfloat; 9]),
    VD8([GLfloat; 108]),
    UV8([GLfloat; 72])
}

impl VertexData {
    /// wrapper for 'get(index)'
    pub fn get(&self, index: usize) -> Option<&GLfloat> {
        match self {
            Self::VD3(data) => { return data.get(index); }
            Self::VD8(data) => { return data.get(index); }
            Self::UV8(data) => { return data.get(index); }
        }
    }
    /// wrapper for 'get_mut(index)'
    #[allow(dead_code)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut GLfloat> {
        match self {
            Self::VD3(data) => { return data.get_mut(index); }
            Self::VD8(data) => { return data.get_mut(index); }
            Self::UV8(data) => { return data.get_mut(index); }
        }
    }
    /// wrapper for 'len()'
    pub fn len(&self) -> usize {
        match self {
            Self::VD3(data) => { return data.len(); }
            Self::VD8(data) => { return data.len(); }
            Self::UV8(data) => { return data.len(); }
        }
    }
    /// creates cube vertex data from center position and size
    pub fn cube_from_pos(center: [GLfloat; 3], half_edge_length: GLfloat) -> (VertexData, VertexData) {
        let mut vertex_data: [GLfloat; 108] = [
            // front face
            -1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0,
            1.0,-1.0, 1.0,
            1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0,
            1.0,-1.0, 1.0,
            // back face
            1.0, 1.0,-1.0,
            -1.0,-1.0,-1.0,
            -1.0, 1.0,-1.0,
            1.0, 1.0,-1.0,
            1.0,-1.0,-1.0,
            -1.0,-1.0,-1.0,
            // left face
            -1.0,-1.0,-1.0,
            -1.0,-1.0, 1.0,
            -1.0, 1.0, 1.0,
            -1.0,-1.0,-1.0,
            -1.0, 1.0, 1.0,
            -1.0, 1.0,-1.0,
            // right face
            1.0, 1.0, 1.0,
            1.0,-1.0,-1.0,
            1.0, 1.0,-1.0,
            1.0,-1.0,-1.0,
            1.0, 1.0, 1.0,
            1.0,-1.0, 1.0,
            // top face
            1.0, 1.0, 1.0,
            1.0, 1.0,-1.0,
            -1.0, 1.0,-1.0,
            1.0, 1.0, 1.0,
            -1.0, 1.0,-1.0,
            -1.0, 1.0, 1.0,
            // bottom face
            1.0,-1.0, 1.0,
            -1.0,-1.0,-1.0,
            1.0,-1.0,-1.0,
            1.0,-1.0, 1.0,
            -1.0,-1.0, 1.0,
            -1.0,-1.0,-1.0];
        for (i, vertex) in vertex_data.iter_mut().enumerate() {
            *vertex = *vertex * half_edge_length + center[i % 3];
        }
        let mut uv_data: [GLfloat; 72] = [
            // Front face
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 1.0,
            // Back face
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            // Left face
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            // Right face
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            1.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            // Top face
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            // Bottom face
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0
        ];
        for uv in uv_data.iter_mut() {
            *uv *= half_edge_length;
        }

        (VertexData::VD8(vertex_data), VertexData::UV8(uv_data))
    }
}


/// a mesh that can be rendered in gl
pub trait Mesh {
    /// creates a new mesh from vertex data
    fn new(vertex_data: VertexData, color_data: VertexData) -> Self where Self: Sized;
    /// draws the mesh
    fn draw(&self, camera: &CameraConfig);
}


/// collection of all meshes
pub struct Meshes {
    pub meshes: Vec<Box<dyn Mesh>>
}

impl Meshes {
    /// draws all meshes
    pub fn draw_all(&self, camera: &CameraConfig) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }
        for mesh in self.meshes.iter() {
            mesh.draw(camera);
        }
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }
}


/// shader program to use to render
pub struct ShaderProgram {
    pub id: GLuint,
    pub mvp_unif: GLint,
    pub sampler_unif: GLint,
    pub pos_attr: GLint,
    pub tex_attr: GLint
}

impl ShaderProgram {
    /// creates new shader program
    pub unsafe fn new(vs_path: &str, fs_path: &str) -> Self {
        let vs_file = read_to_string(vs_path).unwrap();
        let fs_file = read_to_string(fs_path).unwrap();
        let vs = compile_shader(vs_file.as_str(), gl::VERTEX_SHADER);
        let fs = compile_shader(fs_file.as_str(), gl::FRAGMENT_SHADER);
        let id = link_program(vs, fs);

        let c_mvp = CString::new("MVP").unwrap();
        let mvp_unif = gl::GetUniformLocation(id, c_mvp.as_ptr());

        let c_sampler = CString::new("tex_sampler").unwrap();
        let sampler_unif = gl::GetUniformLocation(id, c_sampler.as_ptr());

        let c_out_color = CString::new("out_color").unwrap();
        gl::BindFragDataLocation(id, 0, c_out_color.as_ptr());

        let c_position = CString::new("position").unwrap();
        let pos_attr = gl::GetAttribLocation(id, c_position.as_ptr());

        let c_uv = CString::new("vertexUV").unwrap();
        let tex_attr = gl::GetAttribLocation(id, c_uv.as_ptr());

        Self { id, mvp_unif, sampler_unif, pos_attr, tex_attr }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
