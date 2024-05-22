use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::{mem, ptr};
use crate::meshes::*;
use std::env::current_dir;
use stb_image::image::{Image, LoadResult};
use nalgebra_glm as glm;


/// stores the current camera config for 3d rendering
pub struct CameraConfig {
    pub pos: glm::TVec3<f32>,
    pub focus: glm::TVec3<f32>,
    pub fov: f32,
    pub mvp: glm::Mat4
}

impl CameraConfig {
    /// creates new config with default values
    pub fn new() -> Self {
        let fov = 45.0_f32.to_radians();
        let projection = glm::perspective::<f32>(crate::SCREEN_WIDTH as f32 / crate::SCREEN_HEIGHT as f32, fov, 0.1, 100.0);
        let pos = glm::TVec3::new(4.0, 3.0, 3.0);
        let focus = glm::TVec3::zeros();
        let up = glm::TVec3::y_axis();
        let view = glm::look_at::<f32>(&pos, &focus, &up);
        let mvp = projection * view * glm::Mat4::identity();

        Self {
            pos,
            focus,
            fov,
            mvp
        }
    }
}



fn get_full_res_path(file_path: &str) -> String {
    let full_path = current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            .replace("\\", "/") + "/res/" + file_path;

    return full_path;
}

fn get_texture_path(file_name: &str) -> String {
    get_full_res_path("textures/") + file_name
}

pub fn load_texture(file_name: &str) -> GLuint {
    let mut tex_id = 0;

    let texture: Image<u8>;
    match stb_image::image::load_with_depth(get_texture_path(file_name), 3, false) {
        LoadResult::ImageU8(im) => { texture = im; }
        _ => { panic!("error reading texture") }
    }

    unsafe {
        gl::GenTextures(1, &mut tex_id);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, texture.width as GLint, texture.height as GLint, 0, gl::RGB, gl::UNSIGNED_BYTE, texture.data.as_ptr() as *const GLvoid);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
    }

    tex_id
}


/// cube mesh
pub struct Cube {
    program: ShaderProgram,
    vao: GLuint,
    vbo: GLuint,
    vertex_data: VertexData,
    tex_id: GLuint,
    tbo: GLuint
}


impl Mesh for Cube {
    fn new(vertex_data: VertexData, uv_data: VertexData) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let tex_id = load_texture("wall.png");
        let mut tbo = 0;
        let program: ShaderProgram;

        unsafe {
            program = ShaderProgram::new("./res/shaders/vs.glsl", "./res/shaders/fs.glsl");

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut tbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(vertex_data.get(0).unwrap()),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, tbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (uv_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(uv_data.get(0).unwrap()),
                gl::DYNAMIC_DRAW,
            );
        }

        Cube { program, vao, vbo, vertex_data, tex_id, tbo }
    }


    fn draw(&self, camera: &CameraConfig) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::UseProgram(self.program.id);
            gl::UniformMatrix4fv(self.program.mvp_unif, 1, gl::FALSE, &camera.mvp[0]);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::Uniform1i(self.program.sampler_unif, 0);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(
                self.program.pos_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(self.program.pos_attr as GLuint);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::VertexAttribPointer(
                self.program.tex_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(self.program.tex_attr as GLuint);

            gl::DrawArrays(gl::TRIANGLES, 0, (self.vertex_data.len() / 3) as GLsizei);
            gl::DisableVertexAttribArray(self.program.pos_attr as GLuint);
            gl::DisableVertexAttribArray(self.program.tex_attr as GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}


impl Drop for Cube {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.tbo);
            gl::DeleteTextures(1, &self.tex_id);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
