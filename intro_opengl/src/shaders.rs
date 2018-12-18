/*
    Intro to OpenGL
    Copyright (C) 2018 Copyleft Games

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program; if not, see http://www.gnu.org/licenses
*/


extern crate gleam;
use gleam::gl::Gl;
use std::os::raw::c_int;
use std::rc::Rc;


pub struct ShaderProgram {
    gl: Rc<Gl>,
    program: u32,
    mvp_matrix_uniform: c_int
}

impl ShaderProgram {
    pub fn new(gl: Rc<Gl>,
           vsrc: &'static [u8],
           fsrc: &'static [u8]) -> ShaderProgram {
        let mut status : [i32; 1] = [0];
        let vshader = gl.create_shader(gleam::gl::VERTEX_SHADER);
        gl.shader_source(vshader, &[vsrc]);
        gl.compile_shader(vshader);
        unsafe {
            gl.get_shader_iv(vshader,
                             gleam::gl::COMPILE_STATUS,
                             &mut status);
        }
        if status[0] == 0 {
            panic!("Vertex shader failed to compile")
        }

        let fshader = gl.create_shader(gleam::gl::FRAGMENT_SHADER);
        gl.shader_source(fshader, &[fsrc]);
        gl.compile_shader(fshader);
        unsafe {
            gl.get_shader_iv(fshader,
                             gleam::gl::COMPILE_STATUS,
                             &mut status);
        }
        if status[0] == 0 {
            panic!("Fragment shader failed to compile")
        }
        
        let program = gl.create_program();
        gl.attach_shader(program, vshader);
        gl.attach_shader(program, fshader);
        gl.bind_attrib_location(program, 0, "position");
        gl.link_program(program);
        unsafe {
            gl.get_program_iv(program,
                              gleam::gl::LINK_STATUS,
                              &mut status);
        }
        if status[0] == 0 {
            panic!("Shader program failed to link")
        }

        // Cleanup
        gl.detach_shader(program, vshader);
        gl.delete_shader(vshader);
        gl.detach_shader(program, fshader);
        gl.delete_shader(fshader);

        // Get uniforms
        let mvp_matrix_uniform = gl.get_uniform_location(program, "mvp_matrix");

        ShaderProgram {gl, program, mvp_matrix_uniform}
    }

    pub fn use_program(&self) {
        let program = self.program;
        self.gl.use_program(program);
    }
    pub fn set_mvp_matrix(&self, transpose: bool, mvp_matrix: &[f32]) {
        let mvp_matrix_uniform = self.mvp_matrix_uniform;
        self.gl.uniform_matrix_4fv(mvp_matrix_uniform, transpose, mvp_matrix);
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.gl.delete_program(self.program);
    }
}
