#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
enum GfxCommandTypes {
    Draw,
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

struct Gfx {
    num_commands: usize,
    commands:          [ GfxCommand; 100 ],
    programs:          Vec< glium::Program >,
    indices:           Vec< glium::IndexBuffer<u16> >,
    line_vertices:     glium::VertexBuffer<GfxLineVertex>,
    triangle_vertices: glium::VertexBuffer<GfxTriangleVertex>
}

impl GfxCommand {
    fn noop() -> GfxCommand {
        GfxCommand { flags: 0,
                     command: GfxCommandTypes::NoOp } 
    }
}

impl Gfx {
    fn new(line_vertices: glium::VertexBuffer<GfxLineVertex>, 
           triangle_vertices: glium::VertexBuffer<GfxTriangleVertex>, 
           indices: glium::IndexBuffer<u16>) -> Gfx {
        let programs = Vec::new();
        let indices  = Vec::new();
        Gfx { line_vertices:     line_vertices,
              triangle_vertices: triangle_vertices,
              num_commands:      0,
              programs:          programs,
              indices:           indices,
              commands:          [GfxCommand::noop(); 100] }
    }
    fn add_rotation(&mut self, angle: f32) -> usize {
        self.commands[self.num_commands] = GfxCommand { flags:0, 
                                                        command:GfxCommandTypes::Rotate ( angle ) };
        self.num_commands += 1;
        return self.num_commands - 1;
    }

    fn change_rotation(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::Rotate ( angle );
    }

    fn add_scale(&mut self, scale: f32) -> usize {
        self.commands[self.num_commands] = GfxCommand { flags:0, 
                                                        command:GfxCommandTypes::Scale ( scale ) };
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    
    fn add_translation(&mut self, x: f32, y: f32) -> usize {
        self.commands[self.num_commands] = GfxCommand { flags:0, 
                                                        command:GfxCommandTypes::Translate { x:x, y:y } };
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    
    fn add_draw(&mut self) -> usize {
        self.commands[self.num_commands] = GfxCommand { flags:0, 
                                                        command:GfxCommandTypes::Draw };
        self.num_commands += 1;
        return self.num_commands - 1;
    }

    fn run(&mut self, display: &glium::Display) {
        let mut target = display.draw();
        let mut cur_program = 0usize;
        let mut cur_translation = [ 0.0, 0.0f32 ];
        let mut cur_scale = 0.5f32;
        let mut cur_angle = 0.0f32;
        let mut cur_indices = 0usize;

        target.clear_color(0.0, 0.0, 0.0, 0.0);
        for command in self.commands.iter() {
            match command.command {
                GfxCommandTypes::Draw => {
                    target.draw(&self.line_vertices, &self.indices[cur_indices], &self.programs[cur_program], 
                                &uniform! {translation: cur_translation, scale:cur_scale, angle:cur_angle}, 
                                &Default::default()).unwrap(); },
                GfxCommandTypes::NoOp => { },
                GfxCommandTypes::Rotate(angle) => cur_angle = angle,
                GfxCommandTypes::Scale(scale) => cur_scale = scale,
                GfxCommandTypes::Translate { x, y } => { cur_translation[0] = x; cur_translation[1] = y }
            }
        }
    

        //target.clear_color(0.0, 0.0, 0.0, 0.0);
        //target.draw(&self.line_vertices, &self.indices[cur_indices], &self.programs[cur_program], 
        //            &uniform! {translation: cur_translation, scale:cur_scale, angle:cur_angle}, 
        //            &Default::default()).unwrap();
        target.finish().unwrap();
    }
    fn add_program(&mut self, display: &glium::Display, vert_shader: &str, frag_shader: &str) {
        self.programs.push(program!(display, 
                                    140 => {vertex:vert_shader, fragment:frag_shader}).unwrap());
    }
    fn add_indices(&mut self, display: &glium::Display, indices: &[u16]) {
        self.indices.push(glium::IndexBuffer::new(display, 
                                                  PrimitiveType::LineLoop,
                                                  indices).unwrap());
    }
}


fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let mut display = glium::Display::new(wb, cb, &event_loop).unwrap();
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
        implement_vertex!(GfxLineVertex, position);

        glium::VertexBuffer::new(&display,
            &[
                GfxLineVertex { position: [-0.5, -0.5]},
                GfxLineVertex { position: [ 0.0,  0.5]},
                GfxLineVertex { position: [ 0.5, -0.5]},
                GfxLineVertex { position: [ 0.8, -0.8]},
            ]
        ).unwrap()
    };
    
    // building the vertex buffer, which contains all the vertices that we will draw
    let triangle_buffer = {
        implement_vertex!(GfxTriangleVertex, position, color);

        glium::VertexBuffer::new(&display,
            &[
                GfxTriangleVertex { position: [-0.5, -0.5], color: [0.5,0.5,0.5]},
                GfxTriangleVertex { position: [ 0.0,  0.5], color: [0.5,0.5,0.5]},
                GfxTriangleVertex { position: [ 0.5, -0.5], color: [0.5,0.5,0.5]},
                GfxTriangleVertex { position: [ 0.8, -0.8], color: [0.5,0.5,0.5]},
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

    
    let mut gfx = Gfx::new(vertex_buffer, 
                           triangle_buffer,
                           index_buffer);
    let mut angle = 0.0f32;

    gfx.add_program(&display, vertex140, fragment140);
    gfx.add_indices(&display, &[0u16, 1, 2, 3]);
    gfx.add_rotation(0.5);
    gfx.add_draw();

    gfx.run(&display);

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        
        angle += 0.01;
        gfx.change_rotation(0,angle);
        gfx.run(&mut display);
    });
}
























