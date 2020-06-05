
//#[macro_use]
//extern crate glium;

pub fn rotate (coord: (f32, f32), angle: f32) -> (f32, f32) {
    let tx = coord.0*angle.cos() - coord.1*angle.sin();
    let ty = coord.0*angle.sin() + coord.1*angle.cos();
    return (tx, ty);
}
pub fn add_points(c1: (f32, f32), c2: (f32, f32)) -> (f32, f32) {
    return (c1.0 + c2.0, c1.1 + c2.1);
}

pub fn scale_point( point: (f32, f32), scale: f32) -> (f32, f32) {
    return (point.0 * scale, point.1 * scale);
}
pub fn get_distance(c1: (f32, f32), c2: (f32,f32)) -> f32 {
    return ((c1.0-c2.0).powf(2.0) + (c1.1-c2.1).powf(2.0)).sqrt();
}

pub fn get_angle(c1: (f32, f32), c2: (f32,f32)) -> f32 {
    return f32::atan2(c1.0 - c2.0, 
                      c1.1 - c2.1);
}

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

use std::collections::HashMap;

// GFX constants
const GFX_SKIP: u32 = 1;

#[derive(Copy, Clone)]
enum GfxCommandTypes {
    LineDraw,
    TriangleDraw,
    NoOp,
    Program(usize),
    Indices(usize),
    Rotate(f32),
    SceneScale(f32),
    ObjectScale(f32),
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

pub struct Gfx {
    commands:          Vec< GfxCommand >,
    programs:          Vec< glium::Program >,
    indices:           Vec< glium::IndexBuffer<u32> >,
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

    fn print(&mut self) {
        match self.command {
            GfxCommandTypes::LineDraw => {
                println!("line draw");
            },
            GfxCommandTypes::TriangleDraw => {
                println!("triangle draw");
            },
            GfxCommandTypes::NoOp               => { 
                println!("no op");
            },
            GfxCommandTypes::Indices(index)     =>  { 
                println!("indices {0}", index);
            },
            GfxCommandTypes::Program(index)     =>  { 
                println!("program {0}", index);
            },
            GfxCommandTypes::Rotate(angle)      => { 
                println!("rotate {0}", angle);
            },
            GfxCommandTypes::SceneScale(scale)       => { 
                println!("scene scale {0}", scale);
            },
            GfxCommandTypes::ObjectScale(scale)       => { 
                println!("object scale {0}", scale);
            },
            GfxCommandTypes::Translate { x, y } =>  { 
                println!("translate {0} {1}", x, y);
            },
            GfxCommandTypes::Origin { x, y }    =>  { 
                println!("no op {0} {1}", x, y);
            }
        }
    }



}

impl Gfx {
    pub fn new() -> Gfx {
        let line_vertices = None;
        let triangle_vertices = None;
        let programs = Vec::new();
        let indices  = Vec::new();
        let line_backing = Vec::new();
        let triangle_backing = Vec::new();
        let commands = Vec::new();

        Gfx { line_vertices:     line_vertices,
              triangle_vertices: triangle_vertices,
              programs:          programs,
              indices:           indices,
              line_backing:      line_backing,
              triangle_backing:  triangle_backing,
              backing_changed:   false,
              commands:          commands }

    }

    pub fn program(&mut self, program: usize) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Program ( program )});
        return self.commands.len() - 1;
    }

    pub fn skip(&mut self, id: usize) {
        self.commands[id].flags |= GFX_SKIP;
    }
    
    pub fn unskip(&mut self, id: usize) {
        self.commands[id].flags &= !GFX_SKIP;
    }

    pub fn rotate(&mut self, angle: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Rotate ( angle )});
        return self.commands.len() - 1;
    }

    pub fn change_rotation(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::Rotate ( angle );
    }

    pub fn scene_scale(&mut self, scale: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::SceneScale ( scale )});
        return self.commands.len() - 1;
    }
    
    pub fn object_scale(&mut self, scale: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::ObjectScale ( scale )});
        return self.commands.len() - 1;
    }
    
    pub fn change_scene_scale(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::SceneScale ( angle );
    }
    
    pub fn change_object_scale(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::ObjectScale ( angle );
    }
    pub fn translate(&mut self, x: f32, y: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Translate { x:x, y:y }});
        return self.commands.len() - 1;
    }
    
    pub fn change_translation(&mut self, id: usize, trans: (f32, f32)) {
        self.commands[id].command = GfxCommandTypes::Translate { x:trans.0, y:trans.1 };
    }
    
    pub fn origin(&mut self, x: f32, y: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Origin { x:x, y:y }});
        return self.commands.len() - 1;
    }
    pub fn change_origin(&mut self, id: usize, x: f32, y: f32) {
        self.commands[id].command = GfxCommandTypes::Origin { x:x, y:y };
    }
    
    pub fn line_draw(&mut self) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::LineDraw });
        return self.commands.len() - 1;
    }
    
    pub fn triangle_draw(&mut self) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::TriangleDraw });
        return self.commands.len() - 1;
    }
    
    pub fn indices(&mut self, indices: usize) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Indices (indices)});
        return self.commands.len() - 1;
    }

    pub fn run(&mut self, display: &glium::Display) {
        let mut target          = display.draw();
        let mut cur_program     = 0usize;
        let mut cur_translation = [ 0.0, 0.0f32 ];
        let mut cur_origin      = [ 0.0, 0.0f32 ];
        let mut cur_scene_scale = 0.5f32;
        let mut cur_object_scale = 1.0f32;
        let mut cur_angle       = 0.0f32;
        let mut cur_indices     = 0usize;
        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            multisampling: true,
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
            if command.flags & GFX_SKIP == 0 {
                match command.command {
                    GfxCommandTypes::LineDraw => {
                        match self.line_vertices {
                            None => println!("No Line Vertices Set"),
                            Some(ref vertices) => {
                                target.draw(vertices, 
                                            &self.indices[cur_indices], 
                                            &self.programs[cur_program], 
                                            &uniform! {translation:  cur_translation, 
                                                       scene_scale:  cur_scene_scale, 
                                                       object_scale: cur_object_scale, 
                                                       angle:        cur_angle,
                                                       origin:       cur_origin,
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
                                                       scene_scale:  cur_scene_scale, 
                                                       object_scale: cur_object_scale, 
                                                       angle:        cur_angle,
                                                       origin:       cur_origin,
                                                       aspect_ratio: aspect_ratio}, 
                                            &params).unwrap(); 
                            } 
                        } 
                    },
                    GfxCommandTypes::NoOp               => { },
                    GfxCommandTypes::Indices(index)     => cur_indices = index,
                    GfxCommandTypes::Program(index)     => cur_program = index,
                    GfxCommandTypes::Rotate(angle)      => cur_angle = angle,
                    GfxCommandTypes::SceneScale(scale)  => cur_scene_scale = scale,
                    GfxCommandTypes::ObjectScale(scale) => cur_object_scale = scale,
                    GfxCommandTypes::Translate { x, y } => { cur_translation[0] = x; cur_translation[1] = y }
                    GfxCommandTypes::Origin { x, y }    => { cur_origin[0] = x; cur_origin[1] = y }
                }
            }
        }
    

        target.finish().unwrap();
    }
    pub fn add_program(&mut self, 
                       display: &glium::Display, 
                       vert_shader: &str, 
                       frag_shader: &str) -> usize {
        self.programs.push(program!(display, 
                                    140 => {vertex:vert_shader, fragment:frag_shader}).unwrap());
        return self.programs.len() - 1;
    }
    pub fn add_indices(&mut self, 
                       display: &glium::Display, 
                       indices: &[u32], 
                       primitive_type: PrimitiveType) -> usize {
        self.indices.push(glium::IndexBuffer::new(display, 
                                                  primitive_type,
                                                  indices).unwrap());
        return self.indices.len() - 1;
    }

    pub fn triangle_len(&self) -> usize {
        return self.triangle_backing.len();
    }
    
    pub fn line_len(&self) -> usize {
        return self.line_backing.len();
    }
    
    pub fn add_triangle_vertex(&mut self, 
                               position: (f32, f32), 
                               color: (f32, f32, f32, f32)) {
        self.triangle_backing.push( GfxTriangleVertex { position: [position.0, position.1], color: [color.0, color.1, color.2, color.3] });
        self.backing_changed = true;
    }

    pub fn add_line_vertex(&mut self, 
                               position: (f32, f32)) {
                               
        self.line_backing.push( GfxLineVertex { position: [position.0, position.1]});
        self.backing_changed = true;
    }
}


