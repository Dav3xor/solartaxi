#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
enum GfxCommandTypes {
    Draw {
        vertexes_start: u32,
        indexes_start: u32,
        num_vertices: usize,
        num_indexes: usize
    },
    NoOp,
    Rotate(f32),
    Scale(f32),
    Translate {
        x: f32,
        y: f32
    }
}

#[derive(Copy, Clone)]
struct GfxLineVertex {
    position: [f32; 2]
}

#[derive(Copy, Clone)]
struct GfxTriangleVertex {
    position: [f32; 2],
    color:    [f32; 3]
}

#[derive(Copy, Clone)]
struct GfxCommand {
    flags: u32,
    command: GfxCommandTypes
}

struct Gfx<'a> {
    num_commands: usize,
    commands:          [ GfxCommand; 100 ],
    programs:          &'a [ glium::Program ],
    line_vertices:     glium::VertexBuffer<GfxLineVertex>,
    triangle_vertices: glium::VertexBuffer<GfxTriangleVertex>,
    indices:           glium::IndexBuffer<u16>
}

impl GfxCommand {
    fn noop() -> GfxCommand {
        GfxCommand { flags: 0,
                     command: GfxCommandTypes::NoOp } 
    }
}

impl<'a> Gfx<'a> {
    fn new(line_vertices: glium::VertexBuffer<GfxLineVertex>, 
           triangle_vertices: glium::VertexBuffer<GfxTriangleVertex>, 
           indices: glium::IndexBuffer<u16>,
           programs: &[glium::Program]) -> Gfx {
        Gfx { line_vertices: line_vertices,
              triangle_vertices: triangle_vertices,
              indices: indices,
              num_commands: 0,
              programs: programs,
              commands: [GfxCommand::noop(); 100] }
    }
    fn rotate(&mut self, angle: f32) -> usize {
        self.commands[self.num_commands] = GfxCommand { flags:0, 
                                                        command:GfxCommandTypes::Rotate ( angle ) };
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    fn scale(&mut self, scale: f32) -> usize {
        self.commands[self.num_commands] = GfxCommand { flags:0, 
                                                        command:GfxCommandTypes::Scale ( scale ) };
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    fn translate(&mut self, x: f32, y: f32) -> usize {
        self.commands[self.num_commands] = GfxCommand { flags:0, 
                                                        command:GfxCommandTypes::Translate { x:x, y:y } };
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    fn draw(&mut self, display: &mut glium::Display) {
        let mut target = display.draw();
        let mut cur_program = 0usize;
        let mut cur_translation = [ 1.0, 0.0f32 ];
        let mut cur_scale = 1.0f32;
        let mut cur_angle = 0.0f32;
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&self.line_vertices, &self.indices, &self.programs[cur_program], 
                    &uniform! {translation: cur_translation, scale:cur_scale, angle:cur_angle}, 
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }

}


fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let vertex140: &'static str = " #version 140
                                    in vec2 position;
                                    uniform vec2 translation;
                                    uniform float scale;
                                    uniform float angle;
                                    out vec3 vColor;
                                    void main() {
                                        gl_Position = vec4(((position[0]*cos(angle)-position[1]*sin(angle))+translation[0])*scale,
                                                           ((position[0]*sin(angle)+position[1]*cos(angle))+translation[1])*scale, 0.0, 1.0);
                                        vColor = vec3(1.0,0.0,1.0);
                                    }";

    let fragment140: &'static str = " #version 140
                                      in vec3 vColor;
                                      out vec4 f_color;
                                      void main() {
                                          f_color = vec4(vColor, 1.0);
                                      }";




    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2]
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-0.5, -0.5]},
                Vertex { position: [ 0.0,  0.5]},
                Vertex { position: [ 0.5, -0.5]},
                Vertex { position: [ 0.8, -0.8]},
            ]
        ).unwrap()
    };
    
    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::LineLoop,
                                               &[0u16, 1, 2, 3]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: vertex140,
            fragment: fragment140
        },
    ).unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let draw = move || {
        // building the uniforms
        let translation = [ 1.0, 0.0f32 ];
        let scale = 0.2f32;
        let angle = 1.0f32;

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, 
                    &uniform! {translation: translation, scale:scale, angle:angle}, 
                    &Default::default()).unwrap();
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw();

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    draw();
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}
























