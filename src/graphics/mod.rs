extern crate glium;

pub mod mouse;
pub mod window;
mod framerate;

use glium::{DisplayBuild, Surface};

const FRAMERATE_FRAMES: usize = 128;

pub struct Mygraphics<'a> {
    display: glium::backend::glutin_backend::GlutinFacade,
    indices: glium::index::NoIndices,
    program: glium::Program,
    target: glium::Frame,
    draw_parameters: glium::DrawParameters<'a>,
    flushed: bool,
    pub closed: bool,
    pub framerate: framerate::FrameRate,
    pub mouse: mouse::Mouse,
    pub window: window::Window,
}

impl<'a> Mygraphics<'a> {
    pub fn new(title: String) -> Mygraphics<'a> {
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
            uniform float r;
            uniform float zoom_x;
            uniform float zoom_y;

            void main() {
                interpolated_color = color;
                float cr = cos(r);
                float sr = sin(r);
                float px = position[0];
                float py = position[1];
                float nx = (cr*px+sr*py)*zoom_x;
                float ny = (-sr*px+cr*py)*zoom_y;
                gl_Position = vec4(nx+tx,ny+ty, 0.0, 1.0);
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

        let (wx, wy) = match display.get_window().unwrap().get_inner_size_pixels() {
            Some((_x,_y)) => (_x, _y),
            _ => (1,1),
        };

        Mygraphics {
            display: display,
            indices: indices,
            program: program,
            target: target,
            draw_parameters: draw_parameters,
            flushed: true,
            closed: false,
            framerate: framerate::FrameRate::new(FRAMERATE_FRAMES),
            mouse: mouse::Mouse::new(),
            window: window::Window{
                size_pixels_x: wx,
                size_pixels_y: wy,
            }
        }
    }

    pub fn print(&mut self, shape: &glium::VertexBuffer<Vertex>, tx: f32, ty: f32, r: f32, zoom_x: f32, zoom_y: f32) {
        if self.flushed {
            self.target = self.display.draw();
            self.target.clear_color(0.0, 0.0, 0.0, 1.0);
            self.flushed = false;
        }
        let uniforms = uniform! {
            tx: tx,
            ty: ty,
            r: r,
            zoom_x: zoom_x,
            zoom_y: zoom_y,
        };
        self.target.draw(shape, &self.indices, &self.program, &uniforms,
                    &self.draw_parameters).unwrap();
    }

    pub fn flush(&mut self) {
        self.target.set_finish().unwrap();
        self.flushed = true;
        self.framerate.flush();
        self.mouse.cleardiffs();

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Resized(wx, wy) =>
                    {
                        self.mouse.rescale(self.window.size_pixels_x, self.window.size_pixels_y, wx, wy);
                        self.window.size_pixels_x = wx;
                        self.window.size_pixels_y = wy;
                    },
                glium::glutin::Event::Closed => self.closed = true,
                glium::glutin::Event::MouseMoved((x,y)) => self.mouse.moved(x,y,self.window.size_pixels_x,self.window.size_pixels_y),
                glium::glutin::Event::MouseWheel(w) =>
                    match w {
                        glium::glutin::MouseScrollDelta::LineDelta(dx,dy) => self.mouse.wheel(dx,dy),
                        _ => (),
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
                    self.mouse.button(mymousebutton, mybuttonstate);
                }
                _ => (),
            }
        }
    }

    pub fn compile(&self, shape: &[Vertex]) -> glium::VertexBuffer<Vertex> {
        glium::VertexBuffer::new(&self.display, &shape).unwrap()
    }

}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4]
}
implement_vertex!(Vertex, position, color);

pub fn get_triangle(g: &Mygraphics) -> glium::VertexBuffer<Vertex> {
    let triangle =
        [
            Vertex { position: [-0.2, -0.2], color: [0.0,1.0,1.0,0.5]},
            Vertex { position: [ 0.0,  0.2], color: [1.0,0.0,1.0,0.5]},
            Vertex { position: [ 0.2, -0.07], color: [1.0,1.0,0.0,0.5]}
        ];
    g.compile(&triangle)
}
