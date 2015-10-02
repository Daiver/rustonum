#[macro_use] extern crate rustonum;

mod geometry_primitives_tests{

use rustonum::geometry_primitives::Vector3f;

#[test]
fn test_vec3f_dot01()
{
    let v1  = Vector3f{x: 5.0, y: 6.0, z: -1.0};
    let v2  = Vector3f{x: 1.0, y: 0.0, z:  2.0};
    let res = v1.dot(v2);
    assert!((res - 3.0).abs() < 0.0001);
}

}
