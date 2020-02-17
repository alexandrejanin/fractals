use std::ffi::CString;
use std::ptr;

use gl::types::*;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_code: CString =
            CString::new(std::fs::read_to_string(vertex_path).unwrap()).unwrap();
        let fragment_code: CString =
            CString::new(std::fs::read_to_string(fragment_path).unwrap()).unwrap();

        let vertex_id: u32;
        let fragment_id: u32;
        let program_id: u32;

        unsafe {
            vertex_id = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_id, 1, &vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex_id);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

            gl::GetShaderiv(vertex_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }

            fragment_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_id, 1, &fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment_id);

            gl::GetShaderiv(fragment_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragment_id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&info_log).unwrap_or_else(|err| {
                        println!("{}", err);
                        std::str::from_utf8(&info_log[..err.valid_up_to()]).unwrap()
                    })
                );
            }

            program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_id);
            gl::AttachShader(program_id, fragment_id);
            gl::LinkProgram(program_id);

            gl::DeleteShader(vertex_id);
            gl::DeleteShader(fragment_id);
        }

        Self { id: program_id }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn uniform_location(&self, name: &str) -> Option<GLint> {
        match CString::new(name) {
            Err(_) => None,
            Ok(c_name) => match unsafe { gl::GetUniformLocation(self.id, c_name.as_ptr()) } {
                -1 => None,
                location => Some(location),
            },
        }
    }

    pub fn set_vec2(&self, name: &str, value: (GLfloat, GLfloat)) -> bool {
        match self.uniform_location(name) {
            None => false,
            Some(location) => unsafe {
                gl::UseProgram(self.id);
                gl::Uniform2f(location, value.0, value.1);
                true
            },
        }
    }

    pub fn set_float(&self, name: &str, value: GLfloat) -> bool {
        match self.uniform_location(name) {
            None => false,
            Some(location) => unsafe {
                gl::UseProgram(self.id);
                gl::Uniform1f(location, value);
                true
            },
        }
    }
}
