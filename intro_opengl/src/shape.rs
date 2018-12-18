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

extern crate gleam;
use gleam::{gl as GL, gl::Gl};


pub struct Shape {
    gl: Rc<Gl>,
    ebo: u32,
    vbo: u32,
    tris: i32
}

impl Shape {
    pub fn new(gl: Rc<Gl>, elems: Vec<u16>, verts: Vec<f32>) -> Shape {
        let buffers = gl.gen_buffers(2);
        let ebo = buffers[0];
        let vbo = buffers[1];
        let tris : i32 = elems.len() as i32 / 3;

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, ebo);
        gl.bind_buffer(GL::ARRAY_BUFFER, vbo);
        GL::buffer_data(
            &*gl,
            GL::ELEMENT_ARRAY_BUFFER,
            &elems,
            GL::STATIC_DRAW);
        GL::buffer_data(
            &*gl,
            GL::ARRAY_BUFFER,
            &verts,
            GL::STATIC_DRAW);
        Shape{gl, ebo, vbo, tris}
    }

    pub fn render(&self) {
        let gl = &self.gl;
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl.bind_buffer(GL::ARRAY_BUFFER, self.vbo);

        gl.vertex_attrib_pointer(0, 3, GL::FLOAT, false, 12, 0); // TODO

        gl.enable_vertex_attrib_array(0);
        gl.draw_elements(GL::TRIANGLES, self.tris, GL::UNSIGNED_SHORT, 0);
    }
}
