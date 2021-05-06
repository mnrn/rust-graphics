use gl::types::*;
use std::os::raw::c_void;

pub struct VertexArray {
    id: u32,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

#[allow(dead_code)]
impl VertexArray {
    pub fn new() -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        VertexArray { id: vao }
    }

    pub unsafe fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        gl::BindVertexArray(self.id);
        gl::DrawArrays(mode, first, count);
        gl::BindVertexArray(0);
    }

    pub unsafe fn draw_elements(
        &self,
        mode: GLenum,
        count: GLsizei,
        indices_type: GLenum,
        offset: usize,
    ) {
        gl::BindVertexArray(self.id);
        gl::DrawElements(
            mode,
            count,
            indices_type,
            (offset * std::mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::BindVertexArray(0);
    }

    pub fn init<F>(&self, cb: F)
    where
        F: FnOnce(),
    {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        cb();

        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
