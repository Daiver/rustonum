#[macro_use] extern crate rustonum;

use rustonum::vector::*;

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

#[test]
fn test_vec3_len01()
{
    let v: Vector3f = Vector3f{values: [1.0, 2.0, 3.0]};
    let len2 = v.length_squared();
    assert!((len2 - 14.0).abs() < 0.0001);
}

#[test]
fn test_vec3_len02()
{
    let v: Vector3f = Vector3f{values: [4.0, 3.0, 0.0]};
    let len2 = v.length();
    assert!((len2 - 5.0).abs() < 0.0001);
}


#[test]
fn test_vec3_cross01()
{
    let v1 = Vector3f{values: [1.0, 2.0, 3.0]};
    let v2 = Vector3f{values: [5.0, 6.0, 7.0]};
    let v3 = v1.cross(v2);

    assert!((v3[0] + 4.0).abs() < 0.0001);
    assert!((v3[1] - 8.0).abs() < 0.0001);
    assert!((v3[2] + 4.0).abs() < 0.0001);
}


#[test]
fn test_vec3_ops01()
{
    let v1 = Vector3f{values: [2.5, 3.5, 1.5]};
    let v2 = Vector3f{values: [5.0, 7.0, 3.0]};
    let v3 = v2 - 2.0 * v1;
    assert!(v3.sum().abs() < 0.001);
}

#[test]
fn test_vec3_ops02()
{
    let v1 = Vector3f{values: [2.0, 3.0, 1.0]};
    let v2 = Vector3f{values: [5.0, 7.0, 3.0]};
    let v3 = Vector3f{values: [1.0, 1.0, 1.0]};
    let v4 = 2.0 * (v1 + v3 * 0.5) - v2;
    assert!(v4.sum().abs() < 0.001);
}
