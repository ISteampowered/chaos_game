extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use na::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use rand::{thread_rng, Rng};

fn main() {
    let mut window = Window::new("chaos_game 1");
    let mut rng = thread_rng();

    window.set_light(Light::StickToCamera);

    let a = Point3::new(-1.0, -1.0, 0.0);
    let b = Point3::new(0.0, 1.0, 0.0);
    let c = Point3::new(1.0, -1.0, 0.0);
    let d = Point3::new(0.0, -1.0/3.0, 1.4);
    let choices = [a,b,c, d];

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);
    let yellow = Point3::new(1.0, 1.0, 0.0);

    let white = Point3::new(1.0, 1.0, 1.0);

    let mut points = Vec::new();

    let mut pp = a;
    let mut choice;

    while points.len() < 50_000 {
        choice = rng.choose(&choices).expect("could not choose point");
        pp = Point3::new( (pp.x + choice.x)/2f32, (pp.y + choice.y)/2f32, (pp.z + choice.z)/2f32);
        //pp = Point3::new( (pp.x + choice.x)/2f32, (pp.y + choice.y)/2f32, (pp.x + choice.y)/2f32); slightly wrong z values but is kinda pretty 
        points.push(pp);
    }

    while window.render() {
        window.draw_point(&a, &red);
        window.draw_point(&b, &green);
        window.draw_point(&c, &blue);
        window.draw_point(&d, &yellow);
        

        for point in points.iter() {
            window.draw_point(&point, &white);
        }
    }
}