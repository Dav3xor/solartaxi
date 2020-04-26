#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

use rand::prelude::*;
use rand_distr::Exp1;

#[derive(Copy, Clone)]
enum GfxCommandTypes {
    LineDraw,
    TriangleDraw,
    NoOp,
    Program(usize),
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
    color:    [f32; 4]
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
        let params = glium::DrawParameters { blend: glium::draw_parameters::Blend::alpha_blending(), .. Default::default() };

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

    fn program(&mut self, program: usize) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Program ( program )});
        return self.commands.len() - 1;
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
    
    fn line_draw(&mut self) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::LineDraw });
        self.num_commands += 1;
        return self.num_commands - 1;
    }
    
    fn triangle_draw(&mut self) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::TriangleDraw });
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
        let params = glium::DrawParameters {
            line_width: Some(2.0),
            ..Default::default()
        };
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
                GfxCommandTypes::LineDraw => {
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
                                        &params).unwrap(); 
                        } 
                    } 
                },
                GfxCommandTypes::TriangleDraw => {
                    match self.triangle_vertices {
                        None => println!("No Triangle Vertices Set"),
                        Some(ref vertices) => {
                            target.draw(vertices, 
                                        &self.indices[cur_indices], 
                                        &self.programs[cur_program], 
                                        &uniform! {translation:  cur_translation, 
                                                   scale:        cur_scale, 
                                                   angle:        cur_angle,
                                                   aspect_ratio: aspect_ratio}, 
                                        &params).unwrap(); 
                        } 
                    } 
                },
                GfxCommandTypes::NoOp               => { },
                GfxCommandTypes::Indices(index)     => cur_indices = index,
                GfxCommandTypes::Program(index)     => cur_program = index,
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
    fn add_indices(&mut self, display: &glium::Display, indices: &[u16], primitive_type: PrimitiveType) {
        self.indices.push(glium::IndexBuffer::new(display, 
                                                  primitive_type,
                                                  indices).unwrap());
    }

    fn ship(&mut self, display: &glium::Display) -> usize {
        let start_vert = self.triangle_backing.len();
        
        // fuselage, top to bottom... tip to tail
        self.triangle_backing.push( GfxTriangleVertex { position: [  0.0, 19.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 0
        self.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 16.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 1
        self.triangle_backing.push( GfxTriangleVertex { position: [  1.0, 16.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 2
        self.triangle_backing.push( GfxTriangleVertex { position: [ -3.0, 7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 3
        self.triangle_backing.push( GfxTriangleVertex { position: [  3.0, 7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 4
        self.triangle_backing.push( GfxTriangleVertex { position: [ -4.0, -4.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 5
        self.triangle_backing.push( GfxTriangleVertex { position: [  4.0, -4.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 6
        self.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, -11.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 7
        self.triangle_backing.push( GfxTriangleVertex { position: [  2.0, -11.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 8
        // left wing, starting with forwardmost point
        self.triangle_backing.push( GfxTriangleVertex { position: [ -3.0, 3.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 9
        self.triangle_backing.push( GfxTriangleVertex { position: [ -6.0, -1.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 10
        self.triangle_backing.push( GfxTriangleVertex { position: [ -12.0, -5.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 11
        self.triangle_backing.push( GfxTriangleVertex { position: [ -12.0, -7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 12
        self.triangle_backing.push( GfxTriangleVertex { position: [ -11.0, -8.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 12
        self.triangle_backing.push( GfxTriangleVertex { position: [ -3.0, -7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 13 
        self.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, -0.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 14 
        // right wing, starting with forwardmost point
        self.triangle_backing.push( GfxTriangleVertex { position: [  3.0, 3.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 15
        self.triangle_backing.push( GfxTriangleVertex { position: [  6.0, -1.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 16 
        self.triangle_backing.push( GfxTriangleVertex { position: [  12.0, -5.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 17 
        self.triangle_backing.push( GfxTriangleVertex { position: [  12.0, -7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 18 
        self.triangle_backing.push( GfxTriangleVertex { position: [  11.0, -8.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 12
        self.triangle_backing.push( GfxTriangleVertex { position: [  3.0, -7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 19 
        self.triangle_backing.push( GfxTriangleVertex { position: [  2.0, -0.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 20 
        // tail
        self.triangle_backing.push( GfxTriangleVertex { position: [  0.0, -6.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 21 
        self.triangle_backing.push( GfxTriangleVertex { position: [  0.0, -14.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 22 
        
        let mut indices = Vec::new();

        // fuselage
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+1);
        indices.push((start_vert as u16)+2);

        indices.push((start_vert as u16)+1);
        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+3);

        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+3);
        indices.push((start_vert as u16)+4);

        indices.push((start_vert as u16)+3);
        indices.push((start_vert as u16)+4);
        indices.push((start_vert as u16)+5);
        
        indices.push((start_vert as u16)+4);
        indices.push((start_vert as u16)+5);
        indices.push((start_vert as u16)+6);

        indices.push((start_vert as u16)+5);
        indices.push((start_vert as u16)+6);
        indices.push((start_vert as u16)+7);
        
        indices.push((start_vert as u16)+6);
        indices.push((start_vert as u16)+7);
        indices.push((start_vert as u16)+8);

        // left wing
        indices.push((start_vert as u16)+10);
        indices.push((start_vert as u16)+9);
        indices.push((start_vert as u16)+15);
        
        indices.push((start_vert as u16)+10);
        indices.push((start_vert as u16)+15);
        indices.push((start_vert as u16)+14);
        
        indices.push((start_vert as u16)+10);
        indices.push((start_vert as u16)+14);
        indices.push((start_vert as u16)+13);
        
        indices.push((start_vert as u16)+10);
        indices.push((start_vert as u16)+13);
        indices.push((start_vert as u16)+12);
        
        indices.push((start_vert as u16)+10);
        indices.push((start_vert as u16)+12);
        indices.push((start_vert as u16)+11);

        // right wing
        indices.push((start_vert as u16)+17);
        indices.push((start_vert as u16)+16);
        indices.push((start_vert as u16)+22);

         
        indices.push((start_vert as u16)+17);
        indices.push((start_vert as u16)+22);
        indices.push((start_vert as u16)+21);
        
        indices.push((start_vert as u16)+17);
        indices.push((start_vert as u16)+21);
        indices.push((start_vert as u16)+20);
        
        indices.push((start_vert as u16)+17);
        indices.push((start_vert as u16)+20);
        indices.push((start_vert as u16)+19);
        
        indices.push((start_vert as u16)+17);
        indices.push((start_vert as u16)+19);
        indices.push((start_vert as u16)+18);
        
        self.add_indices(display, &indices, PrimitiveType::TriangleStrip);
        self.backing_changed = true;
        return 0;
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
        self.add_indices(display, &indices, PrimitiveType::LineLoop);
        self.backing_changed = true;
        return 0;
    }

    fn sky(&mut self,
           display: &glium::Display, 
           inner_radius: f32,
           height: f32,
           num_divisions: u32) -> usize {
        let angle_step = (3.14159*2.0)/(num_divisions as f32);
        let mut indices = Vec::new();
        let start_vert = self.triangle_backing.len();
        for i in 0..(num_divisions) {
            let angle = (i as f32)*angle_step;
            self.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+height),
                                                                       angle.cos()*(inner_radius+height) ],
                                                           color:    [0.1, 0.2, 0.5, 1.0]});

            self.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*inner_radius,
                                                                       angle.cos()*inner_radius ],
                                                           color:    [0.3, 0.4, 0.5, 1.0]});
            indices.push((start_vert as u16)+(i as u16)*2);
            indices.push((start_vert as u16)+(i as u16)*2+1);
        }
        indices.push((start_vert as u16));
        indices.push((start_vert as u16)+1);
        let start_vert = self.triangle_backing.len();
        for i in 0..(num_divisions) {
            let angle = (i as f32)*angle_step;
            self.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+(height*1.8)),
                                                                       angle.cos()*(inner_radius+(height*1.8)) ],
                                                           color:    [0.0, 0.0, 0.0, 1.0]});
            self.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+height),
                                                                       angle.cos()*(inner_radius+height) ],
                                                           color:    [0.1, 0.2, 0.5, 1.0]});

            indices.push((start_vert as u16)+(i as u16)*2);
            indices.push((start_vert as u16)+(i as u16)*2+1);
        }
        indices.push((start_vert as u16));
        indices.push((start_vert as u16)+1);
        self.add_indices(display, &indices, PrimitiveType::TriangleStrip);
        self.backing_changed = true;
        return 0;
    }


    fn mountains(&mut self, 
                 display: &glium::Display, 
                 height_fn: fn(f32) -> f32,
                 inner_radius: f32, 
                 num_divisions: u32 ) -> usize {
        let angle_step = (3.14159*2.0)/(num_divisions as f32);
        let mut indices = Vec::new();
        let start_vert = self.triangle_backing.len();
        for i in 0..(num_divisions) {

            let angle = (i as f32)*angle_step;
            let height_modifier: f32 = thread_rng().sample(Exp1);
            //let height = (((height_modifier as f32) * max_height) / 10.0)+(max_height);
            let height = height_fn(angle);
            self.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+height),
                                                                       angle.cos()*(inner_radius+height) ],
                                                           color:    [0.05, 0.05, 0.1, 1.0]});

            self.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*inner_radius,
                                                                       angle.cos()*inner_radius ],
                                                        color:    [0.2, 0.2, 0.5, 1.0]});
            indices.push((start_vert as u16)+(i as u16)*2);
            indices.push((start_vert as u16)+(i as u16)*2+1);
        }
        indices.push((start_vert as u16));
        indices.push((start_vert as u16)+1);
        self.add_indices(display, &indices, PrimitiveType::TriangleStrip);
        self.backing_changed = true;
        return 0;
    }

}
 

fn tall_mountains(angle: f32) -> f32 {
    let max_height = 3.0f32;
    return (max_height*1.2) + (max_height * (angle*83.0).sin()*0.3) + (max_height * (angle*61.0).cos()*0.3);
}

fn short_mountains(angle: f32) -> f32 {
    let max_height = 1.0f32;
    return (max_height*1.25) + (max_height * (angle*59.0).sin()*0.7) + (max_height * (angle*29.0).cos()*0.7);
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let mut display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let linevertex140: &'static str = " #version 140
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
                                            vColor = vec3(1.0,1.0,1.0);
                                        }";

    let linefragment140: &'static str = " #version 140
                                          in vec3 vColor;
                                          out vec4 f_color;
                                          void main() {
                                              f_color = vec4(vColor, 1.0);
                                          }";
    
    let trivertex140: &'static str = " #version 140
                                       in vec2 position;
                                       in vec4 color;
                                       uniform vec2 translation;
                                       uniform vec2 origin;
                                       uniform float scale;
                                       uniform float angle;
                                       uniform float aspect_ratio;
                                       out vec4 vColor;
                                       void main() {
                                           gl_Position = vec4(((position[0]*cos(angle)-position[1]*sin(angle))+(translation[0]-origin[0]))*scale*aspect_ratio,
                                                              ((position[0]*sin(angle)+position[1]*cos(angle))+(translation[1]-origin[1]))*scale, 0.0, 1.0);
                                           vColor = color;
                                       }";

    let trifragment140: &'static str = " #version 140
                                         in vec4 vColor;
                                         out vec4 f_color;
                                         void main() {
                                             f_color = vec4(vColor);
                                         }";
   
    let mut gfx = Gfx::new();
    let mut angle = 0.0f32;

    gfx.mountains(&display, tall_mountains, 300.0, 1000);
    gfx.mountains(&display, short_mountains, 350.0, 300);
    gfx.sky(&display, 1000.0, 8.0, 200);
    gfx.circle(&display, 400, 1000.0);
    gfx.ship(&display);

    gfx.add_program(&display, linevertex140, linefragment140);
    gfx.add_program(&display, trivertex140, trifragment140);
    gfx.rotate(0.5);
    gfx.scale(0.05);

    // draw sky
    gfx.program(1);
    gfx.translate(0.0,-1000.0);
    gfx.indices(2);
    gfx.triangle_draw();

    // tall mountains
    gfx.translate(0.0,-300.0);
    gfx.indices(0);
    gfx.triangle_draw();

    // hills
    gfx.translate(0.0,-350.0);
    gfx.indices(1);
    gfx.triangle_draw();

    gfx.program(0);
    gfx.translate(0.0,-1000.0);
    gfx.indices(3);
    gfx.line_draw();

    gfx.program(1);
    gfx.translate(0.0,0.0);
    gfx.indices(4);
    gfx.triangle_draw();
    
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
        
        angle += 0.0001;
        gfx.change_rotation(0,angle);
        gfx.run(&mut display);
    });
}
























