extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use na::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use rand::{thread_rng, Rng};
use std::f32;

fn main() {
    let mut window = Window::new("chaos_game 1");
    let mut rng = thread_rng();

    window.set_light(Light::StickToCamera);

    let angle = f32::consts::PI * 2.0 / 5.0;

    let a = Point3::new(0.0, 0.0, 0.0);
    let b = Point3::new(1.0, 0.0, 0.0);
    let c = Point3::new(1.0 + angle.cos(), angle.sin(), 0.0);
    let d = Point3::new(0.5, (5.0+2.0*5.0_f32.sqrt()).sqrt()/2.0, 0.0);
    let e = Point3::new(-angle.cos(), angle.sin(), 0.0);

    
    let choices = [a,b,c,d,e];

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);

    let white = Point3::new(1.0, 1.0, 1.0);

    let mut points = Vec::new();

    let mut pp = a;
    let mut choice;
    let mut previous_1 = 0;
    let point_to_num = |point| {
        if point == a {0}
        else if point == b {1}
        else if point == c {2}
        else if point == d {3}
        else if point == e {4}
        else {
            println!("something went wrong");
            5
            }
    };
    let mut num;

    while points.len() < 20_000 {
        choice = *rng.choose(&choices).expect("could not choose point");
        num = point_to_num(choice);
        // println!("num:{}, previous_1:{}, previous_2:{}, {}, {}",num, previous_1, previous_2 , mod_5(num - previous_1), mod_5(num - previous_2));
        while num == previous_1 {
            choice = *rng.choose(&choices).expect("could not choose point");
            num = point_to_num(choice);
        }
        previous_1 = num;

        pp = Point3::new( (pp.x + choice.x)/2f32, (pp.y + choice.y)/2f32, 0.0);
        points.push(pp);
    }

    while window.render() {
        window.draw_point(&a, &red);
        window.draw_point(&b, &green);
        window.draw_point(&c, &blue);
        window.draw_point(&d, &green);
        window.draw_point(&e, &blue);
        

        for point in points.iter() {
            window.draw_point(&point, &white);
        }
    }
}