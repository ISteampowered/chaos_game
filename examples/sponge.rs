extern crate kiss3d;
extern crate nalgebra as na;
extern crate glfw;

use glfw::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3, Translation3};
use std::mem;
use std::fmt;

pub struct MengerSponge {
    pub cube: SceneNode,
    r: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl MengerSponge {
    pub fn new(window: &mut Window, length: f32, x: f32, y:f32, z:f32) -> MengerSponge {
        let mut cube = window.add_cube(length, length, length);
        cube.append_translation(&Translation3::new(x, y, z));
        cube.set_color(1.0, 0.0, 0.0);
        // cube.set_points_size(10.0);
        // cube.set_lines_width(1.0);
        // cube.set_surface_rendering_activation(false);
        MengerSponge {
            cube: cube,
            r: length,
            x:x,
            y:y,
            z:z,
        }
    }

    pub fn next_generation(&mut self, mut window: &mut Window) -> Vec<MengerSponge> {
        let mut boxes = Vec::new();
        let new_r = self.r/3.0;
        for x in (-1)..2 {
            for y in (-1)..2 {
                for z in (-1)..2 {
                    let sum = (x as i32).abs() + (y as i32).abs() + (z as i32).abs();
                    if sum > 1 {
                        let new_box = MengerSponge::new(&mut window, new_r, self.x+(x as f32)*new_r, self.y+(y as f32)*new_r, self.z+(z as f32)*new_r);
                        boxes.push(new_box);
                    }
                }
            }
        }
        window.remove(&mut self.cube);
        boxes
    }
}

impl fmt::Debug for MengerSponge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MergerSponge{{length: {}, x: {}, y: {}, z: {} }}", self.r, self.x, self.y, self.z)
    }
}

fn main() {
    let mut window = Window::new("Menger sponge");
    let mut c = MengerSponge::new(&mut window, 1.0, 0.0, 0.0, 0.0);
    window.set_light(Light::StickToCamera);
    let mut sponge = vec!(c);


    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        for mut event in window.events().iter() {
            //TODO: replace this match with an 'if let ...' statement
            match event.value{
                WindowEvent::Key(glfw::Key::S, _, Action::Press, _) => {
                    let mut next = Vec::new();
                    for mut box1 in sponge {
                        next.append(&mut box1.next_generation(&mut window));
                    }
                    // println!("{:?}", next);
                    sponge = next;
                },
                _ => { }
            }
        }
    }
}