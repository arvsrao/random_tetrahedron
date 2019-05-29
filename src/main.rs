extern crate rand;

use rand::Rng;
use std::f32::consts::PI;

struct Vec2f {
    x: f32,
    y: f32,
}

/*
draw sample from the uniform distribution on S^2
*see spherepoints_notes.pdf for a derivation of the
algorithm.
*/
fn generate_random_s2_point() -> [f32; 3] {
    let mut rng = rand::thread_rng();

    let (x, y): (f32, f32) = (rng.gen(), rng.gen());
    let phi: f32 = (2.0 * x - 1.0).acos();
    let theta: f32 = 2.0 * PI * y;

    [theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos()]
}

fn det3x3(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> f32 {
    a[0] * (b[1] * c[2] - b[2] * c[1]) - b[0] * (a[1] * c[2] - a[2] * c[1])
        + c[0] * (a[1] * b[2] - a[2] * b[1])
}

fn is_in_tetrahedron() -> bool {
    let mat = [
        generate_random_s2_point(), // [0.0, 0.0, 1.0],
        generate_random_s2_point(),
        generate_random_s2_point(),
        generate_random_s2_point(),
    ];

    let weights: [f32; 4] = [
        det3x3(mat[1], mat[2], mat[3]),
        -det3x3(mat[0], mat[2], mat[3]),
        det3x3(mat[0], mat[1], mat[3]),
        -det3x3(mat[0], mat[1], mat[2]),
    ];

    for idx in 0..3 as usize {
        if (weights[idx] > 0.0) != (weights[idx + 1] > 0.0) {
            return false;
        }
    }

    true
}

fn origin_in_triangle(a: &Vec2f, b: &Vec2f, c: &Vec2f) -> bool {
    /*
       a.x , b.x , c.x
       a.y , b.y , c.y
    */
    let weights: [f32; 3] = [
        a.x * b.y - a.y * b.x,
        a.y * c.x - a.x * c.y,
        b.x * c.y - b.y * c.x,
    ];

    for idx in 0..2 as usize {
        if (weights[idx] > 0.0) != (weights[idx + 1] > 0.0) {
            return false;
        }
    }

    true
}

fn generate_triangle() -> bool {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    for idx in 0..3 {
        let theta: f32 = rng.gen_range(-PI, PI);
        vec.push(Vec2f {
            x: theta.cos(),
            y: theta.sin(),
        });
    }

    origin_in_triangle(&vec[0], &vec[1], &vec[2])
}

fn main() {
    let N: f32 = 1000000.0;
    let mut num: f32 = 0.0;

    for idx in 0..N as usize {
        if is_in_tetrahedron() {
            num += 1.0;
        }
    }

    println!("probability that the tetrahedron determined by 4 randomly choosen points on S^2 contains the origin {:?}. Which is very close to 1/8 ({:?})", num / N, 1.0 / 8.0);
}
