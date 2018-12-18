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
use cgmath::{Deg, Matrix4, PerspectiveFov, Rad};

extern crate gleam;
use gleam::{gl as GL, gl::Gl};

extern crate glutin;
use glutin::GlContext;


#[cfg(target_os = "emscripten")]
pub mod emscripten;

pub mod scenes;
pub mod shape;
pub mod shaders;


struct Program {
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow,
    gl: Rc<Gl>,
    p_matrix: Matrix4<f32>,
    scene: scenes::Scene,
    red: f32,
}

impl Program {
    fn new() -> Program {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("Intro to OpenGL")
            .with_maximized(true);
        let gles = glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (2, 0));
        let context = glutin::ContextBuilder::new().with_gl(gles);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
        let red : f32 = 0.0;

        unsafe { gl_window.make_current().unwrap(); }
        let gl = unsafe { GL::GlesFns::load_with(|s| gl_window.get_proc_address(s) as *const _) };

        // FIXME need to calculate initial p_matrix!

        let scene = scenes::Scene::new(gl.clone());

        let dpi = gl_window.get_hidpi_factor();
        let window_size = gl_window.get_inner_size().unwrap().to_physical(dpi);
        let p_matrix = calc_pmatrix(window_size);

        Program {events_loop, gl_window, gl, p_matrix, scene, red}
    }

    fn up(&mut self) {
        // Simple animation cycle, currently rotates the red value
        self.red += 0.01;
        if self.red > 1.0 { self.red = 0.0 };
    }


    #[cfg(target_os = "emscripten")]
    fn run(&mut self) {
        emscripten::set_main_loop_callback(|| {
            {
                let gl_window = &mut self.gl_window;
                let p_matrix = &mut self.p_matrix;
                self.events_loop.poll_events(|event| {
                    match event {
                        glutin::Event::WindowEvent { event, .. } => match event {
                            glutin::WindowEvent::Resized(logical_size) => {
                                let dpi = gl_window.get_hidpi_factor();
                                let window_size = logical_size.to_physical(dpi);
                                *p_matrix = calc_pmatrix(window_size);
                                gl_window.resize(window_size);
                            },
                            _ => (),
                        },
                        _ => ()
                    }
                });
            } 
            self.draw();
        });
    }

    #[cfg(not(target_os = "emscripten"))]
    fn run(&mut self) {
        let mut running = true;

        while running {
            use std::{thread, time};
            {
                let events_loop = &mut self.events_loop;
                let p_matrix = &mut self.p_matrix;
                let gl_window = &mut self.gl_window;

                events_loop.poll_events(|event| {
                    match event {
                        glutin::Event::WindowEvent { event, .. } => match event {
                            glutin::WindowEvent::CloseRequested => running = false,
                            glutin::WindowEvent::Resized(logical_size) => {
                                let dpi = gl_window.get_hidpi_factor();
                                let window_size = logical_size.to_physical(dpi);
                                *p_matrix = calc_pmatrix(window_size);
                                gl_window.resize(window_size);
                            },
                            _ => (),
                        },
                        _ => ()
                    }
                });
            }
            self.draw();
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
        }
    }


    fn draw(&mut self) {
        let gl = self.gl.clone();

        // Clear buffer (rotates red channel for debugging)
        self.up();
        gl.clear_color(self.red, 0.25, 0.5, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        let vp_matrix = self.p_matrix; // TODO
        self.scene.render(&vp_matrix);

        self.gl_window.swap_buffers().unwrap();
    }

}


fn calc_pmatrix(window_size: glutin::dpi::PhysicalSize) -> Matrix4<f32> {
    let aspect = (window_size.width / window_size.height) as f32;
    let fovy = Rad::from(Deg::<f32>(45.0));
    let near: f32 = 1.0;
    let far: f32 = 2560.0;
    let p_fov = PerspectiveFov::<f32> {fovy, aspect, near, far};
    Matrix4::from(p_fov)
}


fn main() {
    let mut program = Program::new();
    program.run();
}
