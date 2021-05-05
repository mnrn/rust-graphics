use cgmath::Array;
use cgmath::Matrix;
use gl;
use gl::types::*;

use std::fs::File;
use std::io::Read;
use std::str;

#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct Shader {
    pub id: u32,
}

impl Drop for Shader {
    // Deletes the Shader program.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vert_path: &str, frag_path: &str) -> Result<Shader, String> {
        // Vertex
        let mut vert_file =
            File::open(vert_path).unwrap_or_else(|_| panic!("failed to open file: {}", vert_path));
        let mut vert_code = String::new();
        vert_file
            .read_to_string(&mut vert_code)
            .expect("failed to read vertex shader file");

        // Fragment
        let mut frag_file =
            File::open(frag_path).unwrap_or_else(|_| panic!("failed to open file: {}", frag_path));
        let mut frag_code = String::new();
        frag_file
            .read_to_string(&mut frag_code)
            .expect("failed to read fragment shader file");

        let successful: bool;
        let mut shader = Shader { id: 0 };
        unsafe {
            // Compile
            let vert = shader.compile(vert_code.as_str(), gl::VERTEX_SHADER)?;
            let frag = shader.compile(frag_code.as_str(), gl::FRAGMENT_SHADER)?;

            // Link
            let id = gl::CreateProgram();
            gl::AttachShader(id, vert);
            gl::AttachShader(id, frag);
            gl::LinkProgram(id);
            successful = {
                let mut res: GLint = 0;
                gl::GetProgramiv(id, gl::LINK_STATUS, &mut res);
                res != 0
            };

            gl::DeleteShader(vert);
            gl::DeleteShader(frag);

            shader.id = id;
        }

        if successful {
            Ok(shader)
        } else {
            Err(shader.log())
        }
    }

    fn compile(&self, source: &str, shader_type: GLuint) -> Result<u32, String> {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            let ptr: *const u8 = source.as_bytes().as_ptr();
            let ptr_i8: *const i8 = std::mem::transmute(ptr);
            let len = source.len() as GLint;
            gl::ShaderSource(id, 1, &ptr_i8, &len);
        }

        let successful = unsafe {
            let mut res: GLint = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut res);
            res != 0
        };
        if successful {
            Ok(id)
        } else {
            Err(self.log())
        }
    }

    fn log(&self) -> String {
        let mut len = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        }
        assert!(len > 0);

        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetProgramInfoLog(self.id, len, std::ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        };

        match String::from_utf8(buf) {
            Ok(log) => log,
            Err(vec) => panic!("Could not convert log from buffer: {}", vec),
        }
    }
}
