use std::{mem::size_of_val, ptr};
use std::mem::size_of;

use gl::{self, types::*};
use glutin::{Context, dpi::PhysicalSize, PossiblyCurrent};

use crate::shader::Program;

pub struct GL {
    vao: GLuint,
    program: Program,
}

impl GL {
    pub fn new(context: &Context<PossiblyCurrent>) -> Self {
        let gl = gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        let program = Program::new("src/shader.vert", "src/shader.frag");
        program.use_program();

        let vertices: [GLfloat; 18] = [
            1.0, 1.0, 0.0,
            1.0, -1.0, 0.0,
            -1.0, -1.0, 0.0,
            1.0, 1.0, 0.0,
            -1.0, 1.0, 0.0,
            -1.0, -1.0, 0.0,
        ];

//        let triangles: [GLint; 6] = [
//            0, 1, 3,
//            1, 2, 3,
//        ];

        let mut vao = 0;
        let mut vbo = 0;
//        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
//            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

//            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
//            gl::BufferData(
//                gl::ARRAY_BUFFER,
//                (triangles.len() * size_of::<GLint>()) as GLsizeiptr,
//                triangles.as_ptr() as *const _,
//                gl::STATIC_DRAW,
//            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            vao,
            program,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.use_program();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
//            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}
