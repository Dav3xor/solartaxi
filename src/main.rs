#[macro_use]
extern crate glium;

mod asset;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

use std::collections::HashMap;

fn rotate (coord: (f32, f32), angle: f32) -> (f32, f32) {
    let tx = coord.0*angle.cos() - coord.1*angle.sin();
    let ty = coord.0*angle.sin() + coord.1*angle.cos();
    return (tx, ty);
}
fn add_points(c1: (f32, f32), c2: (f32, f32)) -> (f32, f32) {
    return (c1.0 + c2.0, c1.1 + c2.1);
}

fn scale_point( point: (f32, f32), scale: f32) -> (f32, f32) {
    return (point.0 * scale, point.1 * scale);
}
fn get_distance(c1: (f32, f32), c2: (f32,f32)) -> f32 {
    return ((c1.0-c2.0).powf(2.0) + (c1.1-c2.1).powf(2.0)).sqrt();
}

fn get_angle(c1: (f32, f32), c2: (f32,f32)) -> f32 {
    return f32::atan2(c1.0 - c2.0, 
                      c1.1 - c2.1);
}

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

struct Gfx {
    commands:          Vec< GfxCommand >,
    programs:          Vec< glium::Program >,
    indices:           Vec< glium::IndexBuffer<u32> >,
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
const FOOT_CLOSED_ANGLE: f32 = -3.14159/2.0 + 0.45;
const GEAR_STEPS: u32 = 200;


// GFX constants
const GFX_SKIP: u32 = 1;


enum LandingGearState {
    Up,
    Down,
    Opening(u32),
    Closing(u32)
}
struct Planet {
    position: (f32, f32),
    velocity: (f32, f32),
    mass: f32,
    size: f32,
    hills_trans: usize,
    mountains_trans: usize,
    hills_geometry: usize,
    mountains_geometry: usize,
    sky_geometry: usize,
    horizon_geometry: usize,
}

impl Planet {
    fn new(position: (f32, f32),
           mass: f32,
           size: f32,
           gfx_geometry: HashMap<String, usize>) -> Planet {
        Planet { position:           position,
                 velocity:           (0.0, 0.0),
                 mass:               mass,
                 size:               size,
                 hills_trans:        gfx_geometry["hills_trans"],
                 mountains_trans:    gfx_geometry["mountains_trans"],
                 hills_geometry:     gfx_geometry["hills"],
                 mountains_geometry: gfx_geometry["mountains"],
                 sky_geometry:       gfx_geometry["sky"],
                 horizon_geometry:   gfx_geometry["horizon"]
        }
    }

    fn tick(&mut self, gfx: &mut Gfx, angle: f32) {
        gfx.change_translation(self.hills_trans, 
                               (angle.sin()*(self.size - (self.size*0.35)),
                               angle.cos()*(self.size - (self.size*0.35))));
        gfx.change_translation(self.mountains_trans, 
                               (angle.sin()*(self.size - (self.size*0.3)),
                               angle.cos()*(self.size - (self.size*0.3))));
    }

    fn geometry(gfx: &mut Gfx, 
                display: &glium::Display,
                radius: f32) -> HashMap<String, usize> {
        let mut handles = HashMap::new(); 

        handles.insert("horizon".to_string(),      Planet::circle(gfx, &display, 200, radius));
        handles.insert("sky".to_string(),             Planet::sky(gfx, &display, radius, 16.0, 200));
        handles.insert("mountains".to_string(), Planet::mountains(gfx, &display, tall_mountains, radius*0.3, 1000));
        handles.insert("hills".to_string(),     Planet::mountains(gfx, &display, short_mountains, radius*0.35, 300));
    
        // draw sky
        gfx.program(1);
        gfx.translate(0.0,0.0);
        gfx.indices(handles["sky"]);
        gfx.triangle_draw();

        // tall mountains
        handles.insert("mountains_trans".to_string(), gfx.translate(0.0,radius - (0.3 * radius)));
        gfx.indices(handles["mountains"]);
        gfx.triangle_draw();

        // hills
        handles.insert("hills_trans".to_string(), gfx.translate(0.0,radius - (0.35 * radius)));
        gfx.indices(handles["hills"]);
        gfx.triangle_draw();

        gfx.program(0);
        gfx.translate(0.0, 0.0);
        gfx.indices(handles["horizon"]);
        gfx.line_draw();
        
        return handles;
    }

    fn circle(gfx: &mut Gfx, 
              display: &glium::Display, 
              num_verts: u32, 
              radius: f32) -> usize {
        let angle_step = (3.14159*2.0)/(num_verts as f32);
        let mut indices = Vec::new();
        let start_vert = gfx.line_backing.len();
        for i in 0..num_verts {
            let angle = (i as f32)*angle_step;
            gfx.line_backing.push(GfxLineVertex { position: [ angle.sin()*radius,
                                                              angle.cos()*radius ] });
            indices.push((start_vert as u32)+(i as u32));
        }
        gfx.backing_changed = true;
        return gfx.add_indices(display, &indices, PrimitiveType::LineLoop);
    }

    fn sky(gfx: &mut Gfx,
           display: &glium::Display, 
           inner_radius: f32,
           height: f32,
           num_divisions: u32) -> usize {
        let angle_step = (3.14159*2.0)/(num_divisions as f32);
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        for i in 0..(num_divisions) {
            let angle = (i as f32)*angle_step;
            gfx.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+height),
                                                                       angle.cos()*(inner_radius+height) ],
                                                           color:    [0.1, 0.2, 0.5, 1.0]});

            gfx.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*inner_radius,
                                                                       angle.cos()*inner_radius ],
                                                           color:    [0.3, 0.4, 0.5, 1.0]});
            indices.push((start_vert as u32)+(i as u32)*2);
            indices.push((start_vert as u32)+(i as u32)*2+1);
        }
        indices.push(start_vert as u32);
        indices.push((start_vert as u32)+1);
        let start_vert = gfx.triangle_backing.len();
        for i in 0..(num_divisions) {
            let angle = (i as f32)*angle_step;
            gfx.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+(height*1.8)),
                                                                       angle.cos()*(inner_radius+(height*1.8)) ],
                                                           color:    [0.0, 0.0, 0.0, 1.0]});
            gfx.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+height),
                                                                       angle.cos()*(inner_radius+height) ],
                                                           color:    [0.1, 0.2, 0.5, 1.0]});

            indices.push((start_vert as u32)+(i as u32)*2);
            indices.push((start_vert as u32)+(i as u32)*2+1);
        }
        indices.push(start_vert as u32);
        indices.push((start_vert as u32)+1);
        gfx.backing_changed = true;
        return gfx.add_indices(display, &indices, PrimitiveType::TriangleStrip);
    }


    fn mountains(gfx: &mut Gfx, 
                 display: &glium::Display, 
                 height_fn: fn(f32) -> f32,
                 inner_radius: f32, 
                 num_divisions: u32 ) -> usize {
        let angle_step = (3.14159*2.0)/(num_divisions as f32);
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        for i in 0..(num_divisions) {

            let angle  = (i as f32)*angle_step;
            let height = height_fn(angle);
            gfx.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*(inner_radius+height),
                                                                       angle.cos()*(inner_radius+height) ],
                                                           color:    [0.05, 0.05, 0.1, 1.0]});

            gfx.triangle_backing.push(GfxTriangleVertex { position: [ angle.sin()*inner_radius,
                                                                       angle.cos()*inner_radius ],
                                                        color:    [0.2, 0.2, 0.5, 1.0]});
            indices.push((start_vert as u32)+(i as u32)*2);
            indices.push((start_vert as u32)+(i as u32)*2+1);
        }
        indices.push(start_vert as u32);
        indices.push((start_vert as u32)+1);
        gfx.backing_changed = true;
        return gfx.add_indices(display, &indices, PrimitiveType::TriangleStrip);
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

struct PlayerShip {
    position: (f32, f32),
    velocity: (f32, f32),
    angle: f32,
    scale: f32,
    flags: u32,
    gear_state: LandingGearState,
    ship_geometry: usize,
    exhaust_draw: usize,
    left_gear_geometry: usize,
    right_gear_geometry: usize,

    gfx_scale: usize,
    gfx_angle: usize,
    gfx_translation: usize,
    
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
    fn new(gfx_handles: HashMap<String, usize>) -> PlayerShip {


        PlayerShip { position:      (0.0, 1000.0),
                     velocity:      (0.0, 0.0),
                     angle: 0.0f32,
                     scale: 0.05,
                     flags: 0,
                     gear_state: LandingGearState::Down,
                     ship_geometry: gfx_handles["fuselage"],
                     exhaust_draw: gfx_handles["exhaust_draw"],
                     left_gear_geometry: gfx_handles["left_gear"],
                     right_gear_geometry: gfx_handles["right_gear"],
                     
                     gfx_scale: gfx_handles["scale"],
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
                     //gfx_origin: gfx_handles["ship_origin"]
        }
    }

    fn gravity(&mut self, planet: &Planet)
    {
        let distance = get_distance(self.position, planet.position);
        let angle    = get_angle(self.position, planet.position);

        self.scale = 0.2 + (distance-planet.size+0.001)/500.0;
        //self.scale = 0.2;
        //self.scale = 0.08;
        if distance > (planet.size-0.01) {
            let pull = (1.0/(distance.powf(2.0))) * 2000.0;
            self.velocity.0 -= angle.sin() * pull;
            self.velocity.1 -= angle.cos() * pull;
        } else {
            self.velocity.0 = 0.0;
            self.velocity.1 = 0.0;
            self.position.0 = angle.sin()*planet.size;
            self.position.1 = angle.cos()*planet.size;
        }
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

    fn turning_left(&self) -> bool {
        if self.flags & ROTATE_LEFT != 0 {
            return true;
        } else {
            return false;
        }
    }

    fn turning_right(&self) -> bool {
        if self.flags & ROTATE_RIGHT != 0 {
            return true;
        } else {
            return false;
        }
    }

    fn thrusting(&self) -> bool {
        if self.flags & THRUST_ON != 0 {
            return true;
        } else {
            return false;
        }
    }

    fn cycle_gear(&mut self) {
        match self.gear_state {
            LandingGearState::Opening (state) => {
                self.gear_state = LandingGearState::Closing(state);
            },
            LandingGearState::Closing (state) => {
                self.gear_state = LandingGearState::Opening(state); 
            },
            LandingGearState::Up => {
                self.gear_state = LandingGearState::Opening(0);
            },
            LandingGearState::Down => {
                self.gear_state = LandingGearState::Closing(GEAR_STEPS-1);
            }
        }
    }

    fn tick(&mut self, gfx: &mut Gfx) {
        let mut gear_angle = 0.0f32;
        let mut foot_angle = 0.0f32;
        
        self.position = add_points(self.position, self.velocity);

        if self.flags & ROTATE_LEFT != 0 {
            self.angle += 0.01;
        } else if self.flags & ROTATE_RIGHT != 0 {
            self.angle -= 0.01;
        }

        if self.flags & THRUST_ON != 0 {
            self.velocity.0 -= self.angle.sin()*0.00205;
            self.velocity.1 += self.angle.cos()*0.00205;
            gfx.unskip(self.exhaust_draw);
        } else {
            gfx.skip(self.exhaust_draw);
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
        
        gfx.change_object_scale(self.gfx_scale, self.scale);
        gfx.change_translation(self.gfx_translation,self.position);

        // gear legs
        gfx.change_translation(self.gfx_left_gear_translation, 
                               add_points(self.position, scale_point(rotate ((-3.0, -7.0), self.angle), self.scale)));
        gfx.change_translation(self.gfx_right_gear_translation, 
                               add_points(self.position, scale_point(rotate ((3.0, -7.0), self.angle), self.scale)));

        // gear feet
        gfx.change_translation(self.gfx_left_foot_translation, 
                               add_points(self.position, scale_point(add_points(rotate((-3.0, -7.0), self.angle),
                                                                                rotate((-2.0, -5.5), self.angle-gear_angle)), self.scale )));
        gfx.change_translation(self.gfx_right_foot_translation, 
                               add_points(self.position, scale_point(add_points(rotate((3.0, -7.0), self.angle),
                                                                                rotate((2.0, -5.5), self.angle+gear_angle)), self.scale )));

        gfx.change_rotation(self.gfx_left_gear_rotation, self.angle-gear_angle);
        gfx.change_rotation(self.gfx_right_gear_rotation, self.angle+gear_angle);
        
        gfx.change_rotation(self.gfx_left_foot_rotation, self.angle-foot_angle);
        gfx.change_rotation(self.gfx_right_foot_rotation, self.angle+foot_angle);
        
        gfx.change_rotation(self.gfx_angle, self.angle);
    }
    
    fn geometry(gfx: &mut Gfx, display: &glium::Display) -> HashMap<String, usize> {
        let start_vert = gfx.triangle_backing.len();
        let mut handles = HashMap::new(); 

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
        indices.push((start_vert as u32)+0); // nose cone
        indices.push((start_vert as u32)+1);
        indices.push((start_vert as u32)+2);
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+2);
        indices.push((start_vert as u32)+3);

        
        indices.push((start_vert as u32)+4); // forward fuselage left
        indices.push((start_vert as u32)+7);
        indices.push((start_vert as u32)+8);

        indices.push((start_vert as u32)+4);
        indices.push((start_vert as u32)+5);
        indices.push((start_vert as u32)+8);

        indices.push((start_vert as u32)+5); // forward fuselage right
        indices.push((start_vert as u32)+6);
        indices.push((start_vert as u32)+8);

        indices.push((start_vert as u32)+6);
        indices.push((start_vert as u32)+8);
        indices.push((start_vert as u32)+9);

        indices.push((start_vert as u32)+7); // middle fuselage left
        indices.push((start_vert as u32)+10);
        indices.push((start_vert as u32)+11);

        indices.push((start_vert as u32)+7);
        indices.push((start_vert as u32)+8);
        indices.push((start_vert as u32)+11);

        indices.push((start_vert as u32)+8); // middle fuselage right
        indices.push((start_vert as u32)+9);
        indices.push((start_vert as u32)+11);

        indices.push((start_vert as u32)+9);
        indices.push((start_vert as u32)+11);
        indices.push((start_vert as u32)+12);

        indices.push((start_vert as u32)+10); // aft fuselage left
        indices.push((start_vert as u32)+13);
        indices.push((start_vert as u32)+14);
       
        indices.push((start_vert as u32)+10);
        indices.push((start_vert as u32)+11);
        indices.push((start_vert as u32)+14);

        indices.push((start_vert as u32)+11); // aft fuselage right
        indices.push((start_vert as u32)+12);
        indices.push((start_vert as u32)+14);
        
        indices.push((start_vert as u32)+12);
        indices.push((start_vert as u32)+14);
        indices.push((start_vert as u32)+15);

        // left wing
        
        indices.push((start_vert as u32)+17);
        indices.push((start_vert as u32)+16);
        indices.push((start_vert as u32)+22);
        
        indices.push((start_vert as u32)+17);
        indices.push((start_vert as u32)+22);
        indices.push((start_vert as u32)+21);
        
        indices.push((start_vert as u32)+17);
        indices.push((start_vert as u32)+21);
        indices.push((start_vert as u32)+20);
        
        indices.push((start_vert as u32)+17);
        indices.push((start_vert as u32)+20);
        indices.push((start_vert as u32)+19);
        
        indices.push((start_vert as u32)+17);
        indices.push((start_vert as u32)+19);
        indices.push((start_vert as u32)+18);
        // right wing
        indices.push((start_vert as u32)+24);
        indices.push((start_vert as u32)+23);
        indices.push((start_vert as u32)+29);
         
        indices.push((start_vert as u32)+24);
        indices.push((start_vert as u32)+29);
        indices.push((start_vert as u32)+28);
        
        indices.push((start_vert as u32)+24);
        indices.push((start_vert as u32)+28);
        indices.push((start_vert as u32)+27);
        
        indices.push((start_vert as u32)+24);
        indices.push((start_vert as u32)+27);
        indices.push((start_vert as u32)+26);
        
        indices.push((start_vert as u32)+24);
        indices.push((start_vert as u32)+26);
        indices.push((start_vert as u32)+25);

        // cockpit
        indices.push((start_vert as u32)+30);
        indices.push((start_vert as u32)+31);
        indices.push((start_vert as u32)+32);

        indices.push((start_vert as u32)+32);
        indices.push((start_vert as u32)+33);
        indices.push((start_vert as u32)+30);

        // left middle window 
        indices.push((start_vert as u32)+35);
        indices.push((start_vert as u32)+34);
        indices.push((start_vert as u32)+39);
        
        indices.push((start_vert as u32)+35);
        indices.push((start_vert as u32)+39);
        indices.push((start_vert as u32)+38);
        
        indices.push((start_vert as u32)+35);
        indices.push((start_vert as u32)+36);
        indices.push((start_vert as u32)+38);

        indices.push((start_vert as u32)+36);
        indices.push((start_vert as u32)+37);
        indices.push((start_vert as u32)+38);
        
        // left rear window
        indices.push((start_vert as u32)+40);
        indices.push((start_vert as u32)+43);
        indices.push((start_vert as u32)+41);

        indices.push((start_vert as u32)+41);
        indices.push((start_vert as u32)+42);
        indices.push((start_vert as u32)+43);

        // right middle window 
        indices.push((start_vert as u32)+45);
        indices.push((start_vert as u32)+44);
        indices.push((start_vert as u32)+49);
        
        indices.push((start_vert as u32)+45);
        indices.push((start_vert as u32)+49);
        indices.push((start_vert as u32)+48);
        
        indices.push((start_vert as u32)+45);
        indices.push((start_vert as u32)+46);
        indices.push((start_vert as u32)+48);

        indices.push((start_vert as u32)+46);
        indices.push((start_vert as u32)+47);
        indices.push((start_vert as u32)+48);
        
        // right rear window
        indices.push((start_vert as u32)+50);
        indices.push((start_vert as u32)+53);
        indices.push((start_vert as u32)+51);

        indices.push((start_vert as u32)+51);
        indices.push((start_vert as u32)+52);
        indices.push((start_vert as u32)+53);


        // fin
        indices.push((start_vert as u32)+54);
        indices.push((start_vert as u32)+55);
        indices.push((start_vert as u32)+56);

        indices.push((start_vert as u32)+56);
        indices.push((start_vert as u32)+57);
        indices.push((start_vert as u32)+54);

        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);

        handles.insert("fuselage".to_string(), gfx.indices.len()-1);

        // exhaust
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, -11.7 ], color: [ 1.0, 0.3, 0.0, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.7, -13.3 ], color: [ 1.0, 1.0, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.4, -18.4 ], color: [ 0.0, 0.0, 0.0, 0.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.4, -13.5 ], color: [ 1.0, 1.0, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -0.7, -12.0 ], color: [ 1.0, 0.3, 0.0, 1.0 ] }); // 57 

        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 2.0, -11.7 ], color: [ 1.0, 0.3, 0.0, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 2.7, -13.3 ], color: [ 1.0, 1.0, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 2.4, -18.4 ], color: [ 0.0, 0.0, 0.0, 0.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.4, -13.5 ], color: [ 1.0, 1.0, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.7, -12.0 ], color: [ 1.0, 0.3, 0.0, 1.0 ] }); // 57 

        // left exhaust 
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+3);
        indices.push((start_vert as u32)+4);

        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+1);
        indices.push((start_vert as u32)+3);

        indices.push((start_vert as u32)+1);
        indices.push((start_vert as u32)+2);
        indices.push((start_vert as u32)+3);

        // right exhaust
        indices.push((start_vert as u32)+5);
        indices.push((start_vert as u32)+8);
        indices.push((start_vert as u32)+9);

        indices.push((start_vert as u32)+5);
        indices.push((start_vert as u32)+6);
        indices.push((start_vert as u32)+8);

        indices.push((start_vert as u32)+6);
        indices.push((start_vert as u32)+7);
        indices.push((start_vert as u32)+8);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        handles.insert("exhaust".to_string(), gfx.indices.len()-1);
        
        // left landing gear leg
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0, 0.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, 0.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, -1.0 ], color: [ 0.2, 0.3, 0.3, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.8, -6.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.2, -6.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 

        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+1);
        indices.push((start_vert as u32)+2);

        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+2);
        indices.push((start_vert as u32)+3);
        
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+3);
        indices.push((start_vert as u32)+4);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        handles.insert("left_gear".to_string(), gfx.indices.len()-1);
        
        // right landing gear leg
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0, 0.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 0.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, -1.0 ], color: [ 0.2, 0.3, 0.3, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.8, -6.0 ], color: [ 0.1, 0.15, 0.15, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 2.2, -6.0 ], color: [ 0.05, 0.1, 0.1, 1.0 ] }); // 57 
        
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+1);
        indices.push((start_vert as u32)+2);

        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+2);
        indices.push((start_vert as u32)+3);
        
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+3);
        indices.push((start_vert as u32)+4);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        handles.insert("right_gear".to_string(), gfx.indices.len()-1);
        
        
        // left gear foot
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0,  0.5 ], color: [ 0.3, 0.3, 0.3, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, 0.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 1.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -2.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+1);
        indices.push((start_vert as u32)+2);
        
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+2);
        indices.push((start_vert as u32)+3);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        handles.insert("left_foot".to_string(), gfx.indices.len()-1);
        
        // right gear foot
        let mut indices = Vec::new();
        let start_vert = gfx.triangle_backing.len();
        
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ 0.0, 0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, 0.0 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [ -1.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        gfx.triangle_backing.push( GfxTriangleVertex { position: [  2.0, -0.5 ], color: [ 0.5, 0.5, 0.5, 1.0 ] }); // 57 
        
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+1);
        indices.push((start_vert as u32)+2);
        
        indices.push((start_vert as u32)+0);
        indices.push((start_vert as u32)+2);
        indices.push((start_vert as u32)+3);
        
        gfx.add_indices(display, &indices, PrimitiveType::TrianglesList);
        
        handles.insert("right_foot".to_string(), gfx.indices.len()-1);

        

        handles.insert("program".to_string(), gfx.program(1));
        handles.insert("scale".to_string(), gfx.scene_scale(0.05));

        // left landing gear
        handles.insert("left_gear_indices".to_string(), gfx.indices(handles["left_gear"]));
        handles.insert("left_gear_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("left_gear_translation".to_string(), gfx.translate(-3.0,-7.0));
        handles.insert("left_gear_draw".to_string(), gfx.triangle_draw());
        
        // right landing gear
        handles.insert("right_gear_indices".to_string(), gfx.indices(handles["right_gear"]));
        handles.insert("right_gear_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("right_gear_translation".to_string(), gfx.translate(3.0,-7.0));
        handles.insert("right_gear_draw".to_string(), gfx.triangle_draw());
    
        // left landing foot
        handles.insert("left_foot_indices".to_string(), gfx.indices(handles["left_foot"]));
        handles.insert("left_foot_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("left_foot_translation".to_string(), gfx.translate(-5.0,-12.5));
        handles.insert("left_foot_draw".to_string(), gfx.triangle_draw());
        
        // right landing foot
        handles.insert("right_foot_indices".to_string(), gfx.indices(handles["right_foot"]));
        handles.insert("right_foot_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("right_foot_translation".to_string(), gfx.translate(5.0,-12.5));
        handles.insert("right_foot_draw".to_string(), gfx.triangle_draw());
        
        // ship rotation/translate
        handles.insert("ship_rotation".to_string(), gfx.rotate(0.0));
        handles.insert("ship_translation".to_string(), gfx.translate(0.0,0.0));

        handles.insert("ship_indices".to_string(), gfx.indices(handles["fuselage"]));
        handles.insert("ship_draw".to_string(), gfx.triangle_draw());
        
        handles.insert("exhaust_indices".to_string(), gfx.indices(handles["exhaust"]));
        handles.insert("exhaust_draw".to_string(), gfx.triangle_draw());
        
        
        gfx.backing_changed = true;
        return handles;
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

    fn skip(&mut self, id: usize) {
        self.commands[id].flags |= GFX_SKIP;
    }
    
    fn unskip(&mut self, id: usize) {
        self.commands[id].flags &= !GFX_SKIP;
    }

    fn rotate(&mut self, angle: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::Rotate ( angle )});
        return self.commands.len() - 1;
    }

    fn change_rotation(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::Rotate ( angle );
    }

    fn scene_scale(&mut self, scale: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::SceneScale ( scale )});
        return self.commands.len() - 1;
    }
    
    fn object_scale(&mut self, scale: f32) -> usize {
        self.commands.push(GfxCommand { flags:0, command:GfxCommandTypes::ObjectScale ( scale )});
        return self.commands.len() - 1;
    }
    
    fn change_scene_scale(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::SceneScale ( angle );
    }
    
    fn change_object_scale(&mut self, id: usize, angle: f32) {
        self.commands[id].command = GfxCommandTypes::ObjectScale ( angle );
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
    fn change_origin(&mut self, id: usize, x: f32, y: f32) {
        self.commands[id].command = GfxCommandTypes::Origin { x:x, y:y };
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
    fn add_program(&mut self, display: &glium::Display, vert_shader: &str, frag_shader: &str) -> usize {
        self.programs.push(program!(display, 
                                    140 => {vertex:vert_shader, fragment:frag_shader}).unwrap());
        return self.programs.len() - 1;
    }
    fn add_indices(&mut self, display: &glium::Display, indices: &[u32], primitive_type: PrimitiveType) -> usize {
        self.indices.push(glium::IndexBuffer::new(display, 
                                                  primitive_type,
                                                  indices).unwrap());
        return self.indices.len() - 1;
    }
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
                                        uniform float scene_scale;
                                        uniform float object_scale;
                                        uniform float angle;
                                        uniform float aspect_ratio;
                                        out vec3 vColor;
                                       
                                        float posx = position[0] * object_scale;
                                        float posy = position[1] * object_scale;
                                        float sina = sin(angle);
                                        float cosa = cos(angle);

                                        void main() {
                                            gl_Position = vec4(((posx*cosa-posy*sina)+(translation[0]-origin[0]))*scene_scale*aspect_ratio,
                                                               ((posx*sina+posy*cosa)+(translation[1]-origin[1]))*scene_scale, 0.0, 1.0);
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
                                       uniform float scene_scale;
                                       uniform float object_scale;
                                       uniform float angle;
                                       uniform float aspect_ratio;
                                       out vec4 vColor;

                                       float posx = position[0] * object_scale;
                                       float posy = position[1] * object_scale;
                                       float sina = sin(angle);
                                       float cosa = cos(angle);
                                       void main() {
                                           gl_Position = vec4(((posx*cosa-posy*sina)+(translation[0]-origin[0]))*scene_scale*aspect_ratio,
                                                              ((posx*sina+posy*cosa)+(translation[1]-origin[1]))*scene_scale, 0.0, 1.0);
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


    //gfx.mountains(&display, tall_mountains, 300.0, 1000);
    //gfx.mountains(&display, short_mountains, 350.0, 300);
    //gfx.sky(&display, 1000.0, 8.0, 200);
    //gfx.circle(&display, 400, 1000.0);

    gfx.add_program(&display, linevertex140, linefragment140);
    gfx.add_program(&display, trivertex140, trifragment140);
    gfx.scene_scale(0.05);
    gfx.origin(0.0,1000.0);

    let mut planet = Planet::new((0.0, 0.0),
                                 1000.0,
                                 1000.0,
                                 Planet::geometry(&mut gfx, &display, 1000.0));

    let mut player_ship = PlayerShip::new(PlayerShip::geometry(&mut gfx, &display));
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
                glutin::event::WindowEvent::KeyboardInput { input, is_synthetic, ..  } => {
                    if input.scancode == 34 && input.state == glutin::event::ElementState::Released {
                        player_ship.cycle_gear();
                    }

                    if input.state == glutin::event::ElementState::Released {
                        if input.scancode == 105 && player_ship.turning_left() {
                            player_ship.rotate_off();
                        }
                        if input.scancode == 106 && player_ship.turning_right() {
                            player_ship.rotate_off();
                        }
                        if input.scancode == 108 && player_ship.thrusting() {
                            player_ship.thrust_off();
                        }
                    } else { // key pressed
                        if input.scancode == 105 {
                            player_ship.rotate_left();
                        }
                        if input.scancode == 106 {
                            player_ship.rotate_right();
                        }
                        if input.scancode == 108 {
                            player_ship.thrust_on();
                        }
                    }
                
                    println!("key: {0} {1} {2}", input.scancode, 
                                                 if input.state == glutin::event::ElementState::Pressed { "pressed" } else { "released" },
                                                 is_synthetic);
                },
                _ => ()
            },
            _ => ()
        };
        
        let angle = get_angle(planet.position, player_ship.position);
        let distance = get_distance(planet.position, player_ship.position);
        let midpoint = planet.size + ((distance - planet.size)/2.0);

        gfx.change_origin(1, -1.0 * angle.sin()*midpoint, -1.0 * angle.cos()*midpoint);
        gfx.change_scene_scale(0, 0.00005 + (1.0/(distance-planet.size + 10.0))  );
        player_ship.tick(&mut gfx);
        planet.tick(&mut gfx, get_angle(player_ship.position, planet.position));
        player_ship.gravity(&planet);
        gfx.run(&mut display);
    });
}

