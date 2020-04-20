#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
enum GfxCommandTypes {
    Draw,
    NoOp,
    Indices(usize),
    Rotate(f32),
    Scale(f32),
    Translate {
        x: f32,
        y: f32
    },
    Origin {
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
    commands:          Vec< GfxCommand >,
    programs:          Vec< glium::Program >,
    indices:           Vec< glium::IndexBuffer<u16> >,
    line_backing:      Vec< GfxLineVertex >,
    triangle_backing:  Vec< GfxTriangleVertex >,
    line_vertices:     Option<glium::VertexBuffer<GfxLineVertex>>,
    triangle_vertices: Option<glium::VertexBuffer<GfxTriangleVertex>>,
    backing_changed:   bool
}

impl GfxCommand {
    fn noop() -> GfxCommand {
        GfxCommand { flags: 0,
                     command: GfxCommandTypes::NoOp } 
    }
}

impl Gfx {
    fn new() -> Gfx {
        let line_vertices = None;
        let triangle_vertices = None;
        let programs = Vec::new();
        let indices  = Vec::new();
        let line_backing = Vec::new();
        let triangle_backing = Vec::new();
        let commands = Vec::new();

        Gfx { line_vertices:     line_vertices,
              triangle_vertices: triangle_vertices,
              num_commands:      0,
              programs:          programs,
              indices:           indices,
              line_backing:      line_backing,
              triangle_backing:  triangle_backing,
              backing_changed:   false,
              commands:          commands }

    }
    fn rotate(&mut self, angle: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Rotate ( angle )});
        return self.commands.len() - 1;
    }

    fn change_rotation(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::Rotate ( angle );
    }

    fn scale(&mut self, scale: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Scale ( scale )});
        return self.commands.len() - 1;
    }
    
    fn translate(&mut self, x: f32, y: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Translate { x:x, y:y }});
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    
    fn origin(&mut self, x: f32, y: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Origin { x:x, y:y }});
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    
    fn draw(&mut self) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Draw });
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    
    fn indices(&mut self, indices: usize) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Indices (indices)});
        self.num_commands += 1;
        return self.num_commands - 1;
    }

    fn run(&mut self, display: &glium::Display) {
        let mut target          = display.draw();
        let mut cur_program     = 0usize;
        let mut cur_translation = [ 0.0, 0.0f32 ];
        let mut cur_origin      = [ 0.0, 0.0f32 ];
        let mut cur_scale       = 0.5f32;
        let mut cur_angle       = 0.0f32;
        let mut cur_indices     = 0usize;

        if self.backing_changed {
            self.line_vertices = {
                implement_vertex!(GfxLineVertex, position);
                Some(glium::VertexBuffer::new(display, &self.line_backing).unwrap())
            };
            self.triangle_vertices = {
                implement_vertex!(GfxTriangleVertex, position, color);
                Some(glium::VertexBuffer::new(display, &self.triangle_backing).unwrap())
            };
            self.backing_changed = false;
        }

        // set the aspect ratio...
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        for command in self.commands.iter() {
            match command.command {
                GfxCommandTypes::Draw => {
                    match self.line_vertices {
                        None => println!("No Line Vertices Set"),
                        Some(ref vertices) => {
                            target.draw(vertices, 
                                        &self.indices[cur_indices], 
                                        &self.programs[cur_program], 
                                        &uniform! {translation:  cur_translation, 
                                                   scale:        cur_scale, 
                                                   angle:        cur_angle,
                                                   aspect_ratio: aspect_ratio}, 
                                        &Default::default()).unwrap(); 
                        } 
                    } 
                },
                GfxCommandTypes::NoOp               => { },
                GfxCommandTypes::Indices(index)     => cur_indices = index,
                GfxCommandTypes::Rotate(angle)      => cur_angle = angle,
                GfxCommandTypes::Scale(scale)       => cur_scale = scale,
                GfxCommandTypes::Translate { x, y } => { cur_translation[0] = x; cur_translation[1] = y }
                GfxCommandTypes::Origin { x, y }    => { cur_origin[0] = x; cur_origin[1] = y }
            }
        }
    

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
    fn circle(&mut self, display: &glium::Display, num_verts: u32, radius: f32) -> usize {
        let angle_step = (3.14159*2.0)/(num_verts as f32);
        let mut indices = Vec::new();
        let start_vert = self.line_backing.len();
        for i in 0..num_verts {
            let angle = (i as f32)*angle_step;
            self.line_backing.push(GfxLineVertex { position: [ angle.sin()*radius,
                                                               angle.cos()*radius ] });
            indices.push((start_vert as u16)+(i as u16));
        }
        self.add_indices(display, &indices);
        self.backing_changed = true;
        return 0;
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
                                    uniform vec2 origin;
                                    uniform float scale;
                                    uniform float angle;
                                    uniform float aspect_ratio;
                                    out vec3 vColor;
                                    void main() {
                                        gl_Position = vec4(((position[0]*cos(angle)-position[1]*sin(angle))+(translation[0]-origin[0]))*scale*aspect_ratio,
                                                           ((position[0]*sin(angle)+position[1]*cos(angle))+(translation[1]-origin[1]))*scale, 0.0, 1.0);
                                        vColor = vec3(1.0,0.0,1.0);
                                    }";

    let fragment140: &'static str = " #version 140
                                      in vec3 vColor;
                                      out vec4 f_color;
                                      void main() {
                                          f_color = vec4(vColor, 1.0);
                                      }";
    
    let mut gfx = Gfx::new();
    let mut angle = 0.0f32;

    gfx.add_program(&display, vertex140, fragment140);
    gfx.circle(&display, 30, 0.5);
    gfx.rotate(0.5);
    gfx.translate(0.5,0.0);
    gfx.scale(2.0);
    gfx.draw();

    gfx.circle(&display, 30, 0.48);
    gfx.translate(0.46,0.0);
    gfx.indices(1);
    gfx.draw();

    gfx.run(&display);

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        
        angle += 0.001;
        gfx.change_rotation(0,angle);
        gfx.run(&mut display);
    });
}
























