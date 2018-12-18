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


use std::rc::Rc;

extern crate cgmath;
use cgmath::Matrix4;

extern crate gleam;
use gleam::{gl as GL, gl::Gl};

use super::shaders::ShaderProgram;
use super::shape::Shape;

pub struct Scene {
    gl: Rc<Gl>,
    //my_shader: ShaderProgram,
    my_shape: Shape,
}

impl Scene {
    pub fn new(gl: Rc<Gl>) -> Scene {
        /* Create your shaders first
        let my_shader = ShaderProgram::new(
            gl.clone(),

            include_bytes!("shader.glslv"),
            include_bytes!("shader.glslf")
        );
        */

        let my_shape = Shape::new(
            gl.clone(),
            vec![0, 1, 2], // EBO - first triangle 
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] // VBO
        );

        Scene {gl, my_shape}
        //Scene {gl, my_shader, my_shape}
    }

    pub fn render(&self, vp_matrix: &Matrix4<f32>) {
        // Model-View-Projection Matrix
        let mvp_matrix : &[f32; 16] = vp_matrix.as_ref();
        self.gl.depth_mask(true);
        //self.my_shader.use_program();
        //self.my_shader.set_mvp_matrix(false, mvp_matrix);
        self.my_shape.render();
    }
}
