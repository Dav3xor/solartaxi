#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

use rand::prelude::*;
use rand_distr::Exp1;
use std::collections::HashMap;

fn translate (coord: (f32, f32), angle: f32) -> (f32, f32) {
    let tx = coord.0*angle.cos() - coord.1*angle.sin();
    let ty = coord.0*angle.sin() + coord.1*angle.cos();
    return (tx, ty);
}
fn add_trans(c1: (f32, f32), c2: (f32, f32)) -> (f32, f32) {
    return (c1.0 + c2.0, c1.1 + c2.1);
}
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
    commands:          Vec< GfxCommand >,
    programs:          Vec< glium::Program >,
    indices:           Vec< glium::IndexBuffer<u16> >,
    line_backing:      Vec< GfxLineVertex >,
    triangle_backing:  Vec< GfxTriangleVertex >,
    line_vertices:     Option<glium::VertexBuffer<GfxLineVertex>>,
    triangle_vertices: Option<glium::VertexBuffer<GfxTriangleVertex>>,
    backing_changed:   bool
}

const ROTATE_LEFT: u32 = 1;
const ROTATE_RIGHT: u32 = 2;
const THRUST_ON: u32 = 4;

const GEAR_CLOSED_ANGLE: f32 = 1.1;
const FOOT_CLOSED_ANGLE: f32 = -3.14159/2.0;
const GEAR_STEPS: u32 = 200;

enum LandingGearState {
    Up,
    Down,
    Opening(u32),
    Closing(u32)
}

struct PlayerShip {
    position: [f32; 2],
    velocity: [f32; 2],
    angle: f32,
    flags: u32,
    gear_state: LandingGearState,
    ship_geometry: usize,
    left_gear_geometry: usize,
    right_gear_geometry: usize,
    gfx_angle: usize,
    gfx_translation: usize,
    gfx_origin: usize,
    
    gfx_left_gear_translation: usize,
    gfx_right_gear_translation: usize,
    gfx_left_gear_rotation: usize,
    gfx_right_gear_rotation: usize,
    
    gfx_left_foot_translation: usize,
    gfx_right_foot_translation: usize,
    gfx_left_foot_rotation: usize,
    gfx_right_foot_rotation: usize
}

impl PlayerShip {
    fn new(gfx_geometry: Vec<usize>,
           gfx_handles: std::collections::HashMap<String, usize>) -> PlayerShip {
        PlayerShip { position:      [0.0f32, 0.0],
                     velocity:      [0.0f32, 0.0],
                     angle: 0.0f32,
                     flags: 0,
                     gear_state: LandingGearState::Down,
                     ship_geometry: gfx_geometry[0],
                     left_gear_geometry: gfx_geometry[1],
                     right_gear_geometry: gfx_geometry[2],
                    
                     gfx_left_gear_translation: gfx_handles["left_gear_translation"],
                     gfx_right_gear_translation: gfx_handles["right_gear_translation"],
                     gfx_left_gear_rotation: gfx_handles["left_gear_rotation"],
                     gfx_right_gear_rotation: gfx_handles["right_gear_rotation"],
                     
                     gfx_left_foot_translation: gfx_handles["left_foot_translation"],
                     gfx_right_foot_translation: gfx_handles["right_foot_translation"],
                     gfx_left_foot_rotation: gfx_handles["left_foot_rotation"],
                     gfx_right_foot_rotation: gfx_handles["right_foot_rotation"],
                     
                     gfx_angle: gfx_handles["ship_rotation"],
                     gfx_translation: gfx_handles["ship_translation"],
                     gfx_origin: gfx_handles["ship_origin"] }
    }

    fn handles(gfx: &mut Gfx) -> std::collections::HashMap<String, usize> {
        let mut handles = HashMap::new(); 

        handles.insert("program".to_string(), gfx.program(1));

        // left landing gear
        handles.insert("left_gear_indices".to_string(), gfx.indices(5));
        handles.insert("left_gear_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("left_gear_translation".to_string(), gfx.translate(-3.0,-7.0));
        handles.insert("left_gear_draw".to_string(), gfx.triangle_draw());
        
        // right landing gear
        handles.insert("right_gear_indices".to_string(), gfx.indices(6));
        handles.insert("right_gear_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("right_gear_translation".to_string(), gfx.translate(3.0,-7.0));
        handles.insert("right_gear_draw".to_string(), gfx.triangle_draw());
    
        // left landing foot
        handles.insert("left_foot_indices".to_string(), gfx.indices(7));
        handles.insert("left_foot_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("left_foot_translation".to_string(), gfx.translate(-5.0,-12.5));
        handles.insert("left_foot_draw".to_string(), gfx.triangle_draw());
        
        // right landing foot
        handles.insert("right_foot_indices".to_string(), gfx.indices(8));
        handles.insert("right_foot_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("right_foot_translation".to_string(), gfx.translate(5.0,-12.5));
        handles.insert("right_foot_draw".to_string(), gfx.triangle_draw());
        
        // ship rotation/translate/origin
        handles.insert("ship_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("ship_translation".to_string(), gfx.translate(0.0,0.0));
        handles.insert("ship_origin".to_string(), gfx.origin(0.0,0.0));

        handles.insert("ship_indices".to_string(), gfx.indices(4));
        handles.insert("ship_draw".to_string(), gfx.triangle_draw());
        
        return handles;
    } 

    fn thrust_on(&mut self) {
        self.flags |= THRUST_ON;
    }

    fn thrust_off(&mut self) {
        self.flags &= !THRUST_ON;
    }
    
    fn rotate_left(&mut self) {
        self.flags |= ROTATE_LEFT;
    }

    fn rotate_right(&mut self) {
        self.flags |= ROTATE_RIGHT;
    }

    fn rotate_off(&mut self) {
        self.flags &= !(ROTATE_LEFT+ROTATE_RIGHT);
    }
    fn cycle_gear(&mut self) {
        println!("xxx");
        match self.gear_state {
            LandingGearState::Opening (mut state) => {
                println!("===== 1");
                self.gear_state = LandingGearState::Closing(state);
            },
            LandingGearState::Closing (mut state) => {
                println!("===== 2");
                self.gear_state = LandingGearState::Opening(state); 
            },
            LandingGearState::Up => {
                println!("===== 3");
                self.gear_state = LandingGearState::Opening(0);
            },
            LandingGearState::Down => {
                println!("===== 4");
                self.gear_state = LandingGearState::Closing(GEAR_STEPS-1);
            }
        }
    }

    fn tick(&mut self, gfx: &mut Gfx) {
        let mut gear_angle = 0.0f32;
        let mut foot_angle = 0.0f32;

        if self.flags & ROTATE_LEFT != 0 {
            self.angle += 0.01;
        } else if self.flags & ROTATE_RIGHT != 0 {
            self.angle -= 0.01;
        }

        match self.gear_state {
            LandingGearState::Opening(mut state) => {
                state += 1;
                if state >= GEAR_STEPS {
                    self.gear_state = LandingGearState::Down;
                } else {
                    self.gear_state = LandingGearState::Opening(state);
                }
                foot_angle = FOOT_CLOSED_ANGLE - (state as f32)*(FOOT_CLOSED_ANGLE/(GEAR_STEPS as f32));
                gear_angle = GEAR_CLOSED_ANGLE - (state as f32)*(GEAR_CLOSED_ANGLE/(GEAR_STEPS as f32));
            },
            LandingGearState::Closing(mut state) => {
                state -= 1;
                if state == 0 {
                    self.gear_state = LandingGearState::Up;
                } else {
                    self.gear_state = LandingGearState::Closing(state);
                }

                foot_angle = FOOT_CLOSED_ANGLE - (state as f32)*(FOOT_CLOSED_ANGLE/(GEAR_STEPS as f32));
                gear_angle = GEAR_CLOSED_ANGLE - (state as f32)*(GEAR_CLOSED_ANGLE/(GEAR_STEPS as f32));
            },
            LandingGearState::Up => {
                foot_angle = FOOT_CLOSED_ANGLE;
                gear_angle = GEAR_CLOSED_ANGLE;
            },
            LandingGearState::Down => {
                foot_angle = 0.0;
                gear_angle = 0.0;
            }
        }

        gfx.change_translation(self.gfx_left_gear_translation, 
                               translate ((-3.0, -7.0), self.angle));
        gfx.change_translation(self.gfx_right_gear_translation, 
                               translate ((3.0, -7.0), self.angle));

        gfx.change_translation(self.gfx_left_foot_translation, 
                               add_trans(translate ((-3.0, -7.0), self.angle),
                                         translate((-2.0, -5.5), self.angle-gear_angle)));
        gfx.change_translation(self.gfx_right_foot_translation, 
                               add_trans(translate ((3.0, -7.0), self.angle),
                                         translate((2.0, -5.5), self.angle+gear_angle)));

        gfx.change_rotation(self.gfx_left_gear_rotation, self.angle-gear_angle);
        gfx.change_rotation(self.gfx_right_gear_rotation, self.angle+gear_angle);
        
        gfx.change_rotation(self.gfx_left_foot_rotation, self.angle-foot_angle);
        gfx.change_rotation(self.gfx_right_foot_rotation, self.angle+foot_angle);
        
        gfx.change_rotation(self.gfx_angle, self.angle);
    }
    
    fn geometry(gfx: &mut Gfx, display: &glium::Display) -> Vec<usize> {
        let start_vert = gfx.triangle_backing.len();
        let mut geometry_ids = Vec::with_capacity(5); 

        // fuselage, top to bottom... tip to tail
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, 19.0 ], color: [ 0.6, 0.5, 0.5, 1.0 ] }); // 0  nosecone
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 16.0 ], color: [ 0.3, 0.25, 0.2, 1.0 ] }); // 1
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, 16.0 ], color: [ 0.8, 0.6, 0.6, 1.0 ] }); // 2
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.0, 16.0 ], color: [ 0.3, 0.25, 0.2, 1.0 ] }); // 3
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 16.0 ], color: [ 0.2, 0.2, 0.2, 1.0 ] }); // 4  first past nosecone
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, 16.0 ], color: [ 0.4, 0.4, 0.4, 1.0 ] }); // 5  first past nosecone
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.0, 16.0 ], color: [ 0.2, 0.2, 0.2, 1.0 ] }); // 6
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, 7.0 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 7
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, 7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 8
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  2.0, 7.0 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 9
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -4.0, -4.0 ], color: [ 0.3, 0.3, 0.3, 1.0 ] }); // 10
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, -4.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 11
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  4.0, -4.0 ], color: [ 0.3, 0.3, 0.3, 1.0 ] }); // 12
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, -11.0 ], color: [ 0.2, 0.2, 0.2, 1.0 ] }); // 13
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, -11.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 14
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  2.0, -11.0 ], color: [ 0.2, 0.2, 0.2, 1.0 ] }); // 15



        // left wing, starting with forwardmost point
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -3.0, 3.0 ], color: [ 0.3, 0.3, 0.3, 1.0 ] }); // 16
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -6.0, -1.0 ], color: [ 0.25, 0.25, 0.25, 1.0 ] }); // 17
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -12.0, -5.0 ], color: [ 0.4, 0.4, 0.4, 1.0 ] }); // 18
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -12.0, -7.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 19
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -11.0, -8.0 ], color: [ 0.6, 0.6, 0.6, 1.0 ] }); // 20
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.9, -7.0 ], color: [ 0.65, 0.65, 0.65, 1.0 ] }); // 21 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, -0.0 ], color: [ 0.7, 0.7, 0.7, 1.0 ] }); // 22 
        // right wing, starting with forwardmost point
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  3.0, 3.0 ], color: [ 0.3, 0.3, 0.3, 1.0 ] }); // 23
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  6.0, -1.0 ], color: [ 0.25, 0.25, 0.25, 1.0 ] }); // 24 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  12.0, -5.0 ], color: [ 0.4, 0.4, 0.4, 1.0 ] }); // 25 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  12.0, -7.0 ], color: [ 0.6, 0.6, 0.6, 1.0 ] }); // 26 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  11.0, -8.0 ], color: [ 0.6, 0.6, 0.6, 1.0 ] }); // 27
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  2.9, -7.0 ], color: [ 0.65, 0.65, 0.65, 1.0 ] }); // 28 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  2.0, -0.0 ], color: [ 0.7, 0.7, 0.7, 1.0 ] }); // 29 

        // cockpit
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, 8.5 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 30 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.3, 4.6 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 31 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, 5.5 ], color: [ 0.2, 0.2, 0.2, 1.0 ] }); // 32 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.3, 4.6 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 33 
      
        // left middle window
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.2, 5.0 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 34 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.4, 4.2 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 35 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.5, 3.1 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 36 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.9, 2.8 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 37 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.8, 3.7 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 38 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.3, 4.2 ], color: [ 0.12, 0.12, 0.12, 1.0 ] }); // 39 
        
        // left rear window
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.5, 2.8 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 40 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.3, 1.1 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 41 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 1.0 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 42 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.9, 2.5 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 43 

        // right middle window
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.2, 5.0 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 44 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.4, 4.2 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 45 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.5, 3.1 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 46 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.9, 2.8 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 47 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.8, 3.7 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 48 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.3, 4.2 ], color: [ 0.2, 0.2, 0.2, 1.0 ] }); // 49 
        
        // right rear window
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.5, 2.8 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 50 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.3, 1.1 ], color: [ 0.0, 0.0, 0.0, 1.0 ] }); // 51 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  1.0, 1.0 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 52 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.9, 2.5 ], color: [ 0.1, 0.1, 0.1, 1.0 ] }); // 53 

        // tail
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, -6.0 ], color: [ 0.8, 0.8, 0.8, 1.0 ] }); // 54 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.5, -8.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 55 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  0.0, -13.0 ], color: [ 0.8, 0.8, 0.8, 1.0 ] }); // 56 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.5, -8.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        
        let mut indices = Vec::new();

        // fuselage
        indices.push((start_vert as u16)+0); // nose cone
        indices.push((start_vert as u16)+1);
        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+3);

        
        indices.push((start_vert as u16)+4); // forward fuselage left
        indices.push((start_vert as u16)+7);
        indices.push((start_vert as u16)+8);

        indices.push((start_vert as u16)+4);
        indices.push((start_vert as u16)+5);
        indices.push((start_vert as u16)+8);

        indices.push((start_vert as u16)+5); // forward fuselage right
        indices.push((start_vert as u16)+6);
        indices.push((start_vert as u16)+8);

        indices.push((start_vert as u16)+6);
        indices.push((start_vert as u16)+8);
        indices.push((start_vert as u16)+9);

        indices.push((start_vert as u16)+7); // middle fuselage left
        indices.push((start_vert as u16)+10);
        indices.push((start_vert as u16)+11);

        indices.push((start_vert as u16)+7);
        indices.push((start_vert as u16)+8);
        indices.push((start_vert as u16)+11);

        indices.push((start_vert as u16)+8); // middle fuselage right
        indices.push((start_vert as u16)+9);
        indices.push((start_vert as u16)+11);

        indices.push((start_vert as u16)+9);
        indices.push((start_vert as u16)+11);
        indices.push((start_vert as u16)+12);

        indices.push((start_vert as u16)+10); // aft fuselage left
        indices.push((start_vert as u16)+13);
        indices.push((start_vert as u16)+14);
       
        indices.push((start_vert as u16)+10);
        indices.push((start_vert as u16)+11);
        indices.push((start_vert as u16)+14);

        indices.push((start_vert as u16)+11); // aft fuselage right
        indices.push((start_vert as u16)+12);
        indices.push((start_vert as u16)+14);
        
        indices.push((start_vert as u16)+12);
        indices.push((start_vert as u16)+14);
        indices.push((start_vert as u16)+15);

        // left wing
        
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
        // right wing
        indices.push((start_vert as u16)+24);
        indices.push((start_vert as u16)+23);
        indices.push((start_vert as u16)+29);
         
        indices.push((start_vert as u16)+24);
        indices.push((start_vert as u16)+29);
        indices.push((start_vert as u16)+28);
        
        indices.push((start_vert as u16)+24);
        indices.push((start_vert as u16)+28);
        indices.push((start_vert as u16)+27);
        
        indices.push((start_vert as u16)+24);
        indices.push((start_vert as u16)+27);
        indices.push((start_vert as u16)+26);
        
        indices.push((start_vert as u16)+24);
        indices.push((start_vert as u16)+26);
        indices.push((start_vert as u16)+25);

        // cockpit
        indices.push((start_vert as u16)+30);
        indices.push((start_vert as u16)+31);
        indices.push((start_vert as u16)+32);

        indices.push((start_vert as u16)+32);
        indices.push((start_vert as u16)+33);
        indices.push((start_vert as u16)+30);

        // left middle window 
        indices.push((start_vert as u16)+35);
        indices.push((start_vert as u16)+34);
        indices.push((start_vert as u16)+39);
        
        indices.push((start_vert as u16)+35);
        indices.push((start_vert as u16)+39);
        indices.push((start_vert as u16)+38);
        
        indices.push((start_vert as u16)+35);
        indices.push((start_vert as u16)+36);
        indices.push((start_vert as u16)+38);

        indices.push((start_vert as u16)+36);
        indices.push((start_vert as u16)+37);
        indices.push((start_vert as u16)+38);
        
        // left rear window
        indices.push((start_vert as u16)+40);
        indices.push((start_vert as u16)+43);
        indices.push((start_vert as u16)+41);

        indices.push((start_vert as u16)+41);
        indices.push((start_vert as u16)+42);
        indices.push((start_vert as u16)+43);

        // right middle window 
        indices.push((start_vert as u16)+45);
        indices.push((start_vert as u16)+44);
        indices.push((start_vert as u16)+49);
        
        indices.push((start_vert as u16)+45);
        indices.push((start_vert as u16)+49);
        indices.push((start_vert as u16)+48);
        
        indices.push((start_vert as u16)+45);
        indices.push((start_vert as u16)+46);
        indices.push((start_vert as u16)+48);

        indices.push((start_vert as u16)+46);
        indices.push((start_vert as u16)+47);
        indices.push((start_vert as u16)+48);
        
        // right rear window
        indices.push((start_vert as u16)+50);
        indices.push((start_vert as u16)+53);
        indices.push((start_vert as u16)+51);

        indices.push((start_vert as u16)+51);
        indices.push((start_vert as u16)+52);
        indices.push((start_vert as u16)+53);


        // fin
        indices.push((start_vert as u16)+54);
        indices.push((start_vert as u16)+55);
        indices.push((start_vert as u16)+56);

        indices.push((start_vert as u16)+56);
        indices.push((start_vert as u16)+57);
        indices.push((start_vert as u16)+54);

        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);

        geometry_ids.push(gfx.indices.len()-1);

        // left landing gear leg
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0, 0.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, 0.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, -1.0 ], color: [ 0.2, 0.3, 0.3, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.8, -6.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.2, -6.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 

        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+1);
        indices.push((start_vert as u16)+2);

        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+3);
        
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+3);
        indices.push((start_vert as u16)+4);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        geometry_ids.push(gfx.indices.len()-1);
        
        // right landing gear leg
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0, 0.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 0.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, -1.0 ], color: [ 0.2, 0.3, 0.3, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.8, -6.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 2.2, -6.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 
        
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+1);
        indices.push((start_vert as u16)+2);

        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+3);
        
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+3);
        indices.push((start_vert as u16)+4);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        geometry_ids.push(gfx.indices.len()-1);
        
        
        // left gear foot
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0,  0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, 0.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+1);
        indices.push((start_vert as u16)+2);
        
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+3);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        geometry_ids.push(gfx.indices.len()-1);
        
        // right gear foot
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0, 0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 0.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  2.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+1);
        indices.push((start_vert as u16)+2);
        
        indices.push((start_vert as u16)+0);
        indices.push((start_vert as u16)+2);
        indices.push((start_vert as u16)+3);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        geometry_ids.push(gfx.indices.len()-1);

        
        
        
        gfx.backing_changed = true;
        return geometry_ids;
    }


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
            GfxCommandTypes::Scale(scale)       => { 
                println!("scale {0}", scale);
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
        return self.commands.len() - 1;
    }
    
    fn change_translation(&mut self, id: usize, trans: (f32, f32)) {
        self.commands[id].command = GfxCommandTypes::Translate { x:trans.0, y:trans.1 };
    }
    
    fn origin(&mut self, x: f32, y: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Origin { x:x, y:y }});
        return self.commands.len() - 1;
    }
    
    fn line_draw(&mut self) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::LineDraw });
        return self.commands.len() - 1;
    }
    
    fn triangle_draw(&mut self) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::TriangleDraw });
        return self.commands.len() - 1;
    }
    
    fn indices(&mut self, indices: usize) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Indices (indices)});
        return self.commands.len() - 1;
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


    let mut player_ship = PlayerShip::new(PlayerShip::geometry(&mut gfx, &display),
                                          PlayerShip::handles(&mut gfx));
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
                },
                glutin::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic  } => {
                    if input.scancode == 34 && input.state == glutin::event::ElementState::Released {
                        player_ship.cycle_gear();
                    }

                    if input.state == glutin::event::ElementState::Released {
                        player_ship.rotate_off();
                    } else if input.scancode == 105 && input.state == glutin::event::ElementState::Pressed {
                        player_ship.rotate_left();
                    } else if input.scancode == 106 && input.state == glutin::event::ElementState::Pressed {
                        player_ship.rotate_right();
                    }
                    println!("key: {0} {1} {2}", input.scancode, 
                                                 if input.state == glutin::event::ElementState::Pressed { "pressed" } else { "released" },
                                                 is_synthetic);
                },
                _ => ()
            },
            _ => ()
        };
        
        angle += 0.0001;
        gfx.change_rotation(0,angle);
        player_ship.tick(&mut gfx);
        gfx.run(&mut display);
    });
}
























