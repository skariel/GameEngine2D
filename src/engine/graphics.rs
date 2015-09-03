// Copyright 2015 Jose Ariel Keselman
//
// This file is part of GameEngine2D.
//
// GameEngine2D is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// GameEngine2D is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GameEngine2D.  If not, see <http://www.gnu.org/licenses/>.

use glium;
use glium::{DisplayBuild, Surface};
use engine::{window, mouse, keyboard, shapes, camera};

pub struct Graphics<'a> {
    display: glium::backend::glutin_backend::GlutinFacade,
    indices: glium::index::NoIndices,
    program: glium::Program,
    target: glium::Frame,
    draw_parameters: glium::DrawParameters<'a>,
    pub window: window::Window,
}

impl<'a> Drop for Graphics<'a> {
    fn drop(&mut self) {
        match self.target.set_finish() {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}

impl<'a> Graphics<'a> {
    pub fn new(title: String) -> Graphics<'a> {
        let display = glium::glutin::WindowBuilder::new()
            .with_vsync()
            .with_title(title)
            .with_multisampling(8)
            .build_glium().unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            in vec4 color;

            out vec4 interpolated_color;

            uniform float tx;
            uniform float ty;
            uniform float camera_x;
            uniform float camera_y;
            uniform float rotation;
            uniform float camera_rotation;
            uniform float zoom_x;
            uniform float zoom_y;
            uniform float camera_zoom_x;
            uniform float camera_zoom_y;
            uniform float aspect_ratio_y;

            void main() {
                interpolated_color = color;
                float cr = cos(rotation);
                float sr = sin(rotation);

                float px = position[0]*zoom_x;
                float py = position[1]*zoom_y;

                float nx = cr*px+sr*py;
                float ny = -sr*px+cr*py;

                nx += tx;
                ny += ty;

                float c_cr = cos(camera_rotation);
                float c_sr = sin(camera_rotation);

                float c_nx = c_cr*nx+c_sr*ny;
                float c_ny = -c_sr*nx+c_cr*ny;

                c_nx *= camera_zoom_x;
                c_ny *= camera_zoom_y;

                c_nx -= camera_x,
                c_ny -= camera_y,

                c_ny /= aspect_ratio_y;

                gl_Position = vec4(c_nx,c_ny, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec4 interpolated_color;
            out vec4 outcolor;

            void main() {
                outcolor = interpolated_color;
            }
        "#;

        let draw_parameters = glium::DrawParameters {
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            blending_function: Some(glium::draw_parameters::BlendingFunction::Addition {
                source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                destination:  glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha
            }),
            dithering: true,
            .. Default::default()
        };

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.set_finish().unwrap();
        target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let (wx, wy) = display.get_framebuffer_dimensions();
        Graphics{
            display: display,
            indices: indices,
            program: program,
            target: target,
            draw_parameters: draw_parameters,
            window: window::Window::new(wx, wy),
        }
    }

    pub fn print(&mut self, camera: &camera::Camera, shape: &glium::VertexBuffer<shapes::Vertex>, tx: f32, ty: f32, rotation: f32, zoom_x: f32, zoom_y: f32) {
        let uniforms = uniform! {
            tx : tx,
            ty : ty,
            camera_x: camera.x,
            camera_y: camera.y,
            rotation: rotation,
            camera_rotation: camera.rotation,
            zoom_x: zoom_x,
            zoom_y: zoom_y,
            camera_zoom_x: camera.zoom_x,
            camera_zoom_y: camera.zoom_y,
            aspect_ratio_y: self.window.aspect_ratio_y,
        };
        self.target.draw(shape, &self.indices, &self.program, &uniforms,
                    &self.draw_parameters).unwrap();
    }

    pub fn compile(&self, shape: &[shapes::Vertex]) -> glium::VertexBuffer<shapes::Vertex> {
        glium::VertexBuffer::new(&self.display, &shape).unwrap()
    }

    pub fn flush(&mut self) {
        self.target.set_finish().unwrap();
        self.target = self.display.draw();
        self.target.clear_color(0.0, 0.0, 0.0, 1.0);
    }

    pub fn poll_events(&mut self, mouse: &mut mouse::Mouse, keyboard: &mut keyboard::Keyboard) {
        mouse.cleardiffs();
        keyboard.cleardiffs();

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Resized(wx, wy) =>
                    {
                        let new_window = window::Window::new(wx, wy);
                        mouse.rescale(&self.window, &new_window);
                        self.window = new_window;
                    },
                glium::glutin::Event::Closed => self.window.closed = true,
                glium::glutin::Event::MouseMoved((x,y)) => mouse.moved(x,y,&self.window),
                glium::glutin::Event::MouseWheel(w) =>
                    if let glium::glutin::MouseScrollDelta::LineDelta(dx,dy) = w {
                        mouse.wheel(dx,dy);
                    },
                glium::glutin::Event::MouseInput(elementstate, mousebutton) => {
                    let mybuttonstate = match elementstate {
                        glium::glutin::ElementState::Pressed => mouse::ButtonState::Pressed,
                        glium::glutin::ElementState::Released => mouse::ButtonState::Released,
                    };
                    let mymousebutton = match mousebutton {
                        glium::glutin::MouseButton::Left => mouse::Button::Left,
                        glium::glutin::MouseButton::Right => mouse::Button::Right,
                        glium::glutin::MouseButton::Middle => mouse::Button::Middle,
                        _ => mouse::Button::Other,
                    };
                    mouse.button(mymousebutton, mybuttonstate);
                },
                glium::glutin::Event::KeyboardInput(gstate, gid, _) => {
                    let state = match gstate {
                        glium::glutin::ElementState::Pressed => keyboard::KeyState::Pressed,
                        glium::glutin::ElementState::Released => keyboard::KeyState::Released,
                    };
                    let id = keyboard::KeyId(gid);
                    keyboard.key(id, state);
                },
                _ => (),
            }
        };

    }
}
