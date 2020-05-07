extern crate quaternions;
use quaternions::vec3::Vec3;
use quaternions::quaternion::Quaternion;

fn main() {
    let test_vec = Vec3::init()
    .x(2.0)
    .y(0.0)
    .z(0.0)
    .build();

    let rot_quat = Quaternion::init_by_axis_angle()
    .y(1.0)
    .angle(std::f64::consts::PI/4.0)
    .normalized(true)
    .build();

    let rotated_vec = test_vec.rotate(rot_quat);

    println!("X: {:.4}, Y: {:.4}, Z: {:.4}", rotated_vec.x(), rotated_vec.y(), rotated_vec.z());
}
