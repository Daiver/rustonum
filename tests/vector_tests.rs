#[macro_use] extern crate rustonum;
//#![cfg_attr(features = "unstable")]

use rustonum::Vector3f;

#[test]
fn test_vec3_ind01()
{
    let v: Vector3f = Vector3f{values: [1.0, 2.0, 3.0]};
    
    assert!((v[0] - 1.0).abs() < 0.0001);
    assert!((v[1] - 2.0).abs() < 0.0001);
    assert!((v[2] - 3.0).abs() < 0.0001);
}

#[test]
#[should_panic]
fn test_vec3_ind02()
{
    let v: Vector3f = Vector3f{values: [1.0, 2.0, 3.0]};
    
    assert!((v[5] - 1.0).abs() < 0.0001);
}
