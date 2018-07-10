extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;
extern crate num;

use na::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use rand::{thread_rng, Rng};
use num::Float;

// fn to_scale<T: Float>(from_lower_bound: T, from_upper_bound: T, to_lower_bound: T, to_upper_bound: T) ->  impl fn(T) -> T {
//     move |n| (n - from_lower_bound)/(from_upper_bound-from_lower_bound) * (to_upper_bound - to_lower_bound) + to_lower_bound
// }

fn distance_to_line<T: Float>(x0: T, y0: T, x1: T, y1: T, x2: T, y2: T) -> T {
    ((y2-y1)*x0 - (x2-x1)*y0 + x2*y1 - y2*x1).abs()/((y2-y1).powi(2) + (x2-x1).powi(2)).sqrt()
}

fn distance<T: Float>(x1: T, y1: T, x2: T, y2: T) -> T {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}


fn main() {
    let mut window = Window::new("chaos_game 1");
    let mut rng = thread_rng();

    window.set_light(Light::StickToCamera);



    let a = Point3::new(-1.0, -1.0, 0.0);
    let b = Point3::new(0.0, 1.0, 0.0);
    let c = Point3::new(1.0, -1.0, 0.0);
    let choices = [a,b,c];

    let max_distance_from_a = distance_to_line(a.x, a.y, b.x, b.y, c.x, c.y);
    let max_distance_from_b = distance_to_line(b.x, b.y, a.x, a.y, c.x, c.y);
    let max_distance_from_c = distance_to_line(c.x, c.y, b.x, b.y, a.x, a.y);
    println!("a: {}, b:{}, c:{}",max_distance_from_a, max_distance_from_b, max_distance_from_c);

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);

    let _white = Point3::new(1.0, 1.0, 1.0);

    let mut points = Vec::new();

    let mut pp = a;
    let mut choice;


    while window.render() {
        window.draw_point(&a, &red);
        window.draw_point(&b, &green);
        window.draw_point(&c, &blue);
        
        if points.len() < 10_000 {
            choice = rng.choose(&choices).expect("could not choose point");
            pp = Point3::new( (pp.x + choice.x)/2f32, (pp.y + choice.y)/2f32, 0.0);
            points.push(pp);
        }

        for point in points.iter() {
            window.draw_point(&point, &Point3::new(
                1.0-distance(point.x, point.y, a.x, a.y)/max_distance_from_a,
                1.0-distance(point.x, point.y, b.x, b.y)/max_distance_from_b,
                1.0-distance(point.x, point.y, c.x, c.y)/max_distance_from_c,
                ));
        }
    }
}