mod geo;

use geo::vec3::Vec3;

fn main() {
    let p = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    println!("{:?}", p);
    println!("{}", p == p);
}
