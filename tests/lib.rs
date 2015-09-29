#[macro_use] extern crate rustonum;


#[cfg(test)]
mod matrix_tests{

use rustonum::MatrixXf;

#[test]
fn test_ones01()
{
    let x = MatrixXf::ones(2, 1);
    assert!((x[(0, 0)] - 1.0).abs() < 0.0001);
    assert!((x[(1, 0)] - 1.0).abs() < 0.0001);
}

#[test]
fn test_consts01()
{
    let x = MatrixXf::consts(2, 1, 6.0);
    assert!((x[(0, 0)] - 6.0).abs() < 0.0001);
    assert!((x[(1, 0)] - 6.0).abs() < 0.0001);
}

#[test]
fn test_ident01()
{
    let x = MatrixXf::ident(2);
    assert!(x.cols() == 2);
    assert!(x.rows() == 2);

    assert!((x[(0, 0)] - 1.0).abs() < 0.0001);
    assert!((x[(1, 0)] - 0.0).abs() < 0.0001);
    assert!((x[(0, 1)] - 0.0).abs() < 0.0001);
    assert!((x[(1, 1)] - 1.0).abs() < 0.0001);
}


#[test]
fn test_from_vec01()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0]);
    assert!((x[(0, 0)] - 1.0).abs() < 0.0001);
    assert!((x[(1, 0)] - 2.0).abs() < 0.0001);
    assert!((x[(2, 0)] - 3.0).abs() < 0.0001);
}

#[test]
fn test_reshape01()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(3, 2);
    assert!((x[(0, 0)] - 1.0).abs() < 0.0001);
    assert!((x[(0, 1)] - 2.0).abs() < 0.0001);
    assert!((x[(1, 0)] - 3.0).abs() < 0.0001);
    assert!((x[(1, 1)] - 4.0).abs() < 0.0001);
    assert!((x[(2, 0)] - 5.0).abs() < 0.0001);
    assert!((x[(2, 1)] - 6.0).abs() < 0.0001);
}

#[test]
fn test_max_min01()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(3, 2);
    assert!((x.max() - 6.0) < 0.001);
    assert!((x.min() - 1.0) < 0.001);
}

#[test]
fn test_add_matrix01()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let y = MatrixXf::ones(2, 3);
    let z = x + y;

    assert!((z[(0, 0)] - 2.0).abs() < 0.0001);
    assert!((z[(0, 1)] - 3.0).abs() < 0.0001);
    assert!((z[(0, 2)] - 4.0).abs() < 0.0001);
    assert!((z[(1, 0)] - 5.0).abs() < 0.0001);
    assert!((z[(1, 1)] - 6.0).abs() < 0.0001);
    assert!((z[(1, 2)] - 7.0).abs() < 0.0001);
}

#[test]
#[should_panic]
fn test_add_matrix02()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let y = MatrixXf::ones(2, 2);
    x + y;
}

#[test]
fn test_add_matrix03()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let y = MatrixXf::ones(2, 3);
    let z = &x + &y;

    assert!((z[(0, 0)] - 2.0).abs() < 0.0001);
    assert!((z[(0, 1)] - 3.0).abs() < 0.0001);
    assert!((z[(0, 2)] - 4.0).abs() < 0.0001);
    assert!((z[(1, 0)] - 5.0).abs() < 0.0001);
    assert!((z[(1, 1)] - 6.0).abs() < 0.0001);
    assert!((z[(1, 2)] - 7.0).abs() < 0.0001);
}

#[test]
fn test_add_matrix_f3201()
{
    let m  = MatrixXf::from_vec(vec![-1.0, 5.0]);
    let m2 = m + 10.0;
    assert!((m2[(0, 0)] - 9.0).abs() < 0.0001);
    assert!((m2[(1, 0)] - 15.0).abs() < 0.0001);
}

#[test]
fn test_add_f32_matrix01()
{
    let m  = MatrixXf::from_vec(vec![-1.0, 5.0]);
    let m2 = -5.0 + m + 10.0;
    assert!((m2[(0, 0)] - 4.0).abs() < 0.0001);
    assert!((m2[(1, 0)] - 10.0).abs() < 0.0001);
}

#[test]
fn test_add_f32_matrix02()
{
    let m  = MatrixXf::from_vec(vec![-1.0, 5.0]);
    let m2 = -5.0 + &m + 10.0;
    assert!((m2[(0, 0)] - 4.0).abs() < 0.0001);
    assert!((m2[(1, 0)] - 10.0).abs() < 0.0001);
}

#[test]
fn test_sub_matrix01()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let y = MatrixXf::ones(2, 3);
    let z = x - y;

    assert!((z[(0, 0)] - 0.0).abs() < 0.0001);
    assert!((z[(0, 1)] - 1.0).abs() < 0.0001);
    assert!((z[(0, 2)] - 2.0).abs() < 0.0001);
    assert!((z[(1, 0)] - 3.0).abs() < 0.0001);
    assert!((z[(1, 1)] - 4.0).abs() < 0.0001);
    assert!((z[(1, 2)] - 5.0).abs() < 0.0001);

}

#[test]
fn test_sub_matrix02()
{
    let x = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let y = MatrixXf::ones(2, 3);
    let z = &x - &y;

    assert!((z[(0, 0)] - 0.0).abs() < 0.0001);
    assert!((z[(0, 1)] - 1.0).abs() < 0.0001);
    assert!((z[(0, 2)] - 2.0).abs() < 0.0001);
    assert!((z[(1, 0)] - 3.0).abs() < 0.0001);
    assert!((z[(1, 1)] - 4.0).abs() < 0.0001);
    assert!((z[(1, 2)] - 5.0).abs() < 0.0001);

}

#[test]
fn test_sub_f32_matrix01()
{
    let m  = MatrixXf::from_vec(vec![-1.0, 5.0]);
    let m2 = 5.0 - m - 10.0;
    //let m2 = 5.0 - m;
    assert!((m2[(0, 0)] + 4.0).abs() < 0.0001);
    assert!((m2[(1, 0)] + 10.0).abs() < 0.0001);
}


#[test]
fn test_sub_f32_matrix02()
{
    let m  = MatrixXf::from_vec(vec![-1.0, 5.0]);
    let m2 = 5.0 - &m - 10.0;
    assert!((m2[(0, 0)] + 4.0).abs() < 0.0001);
    assert!((m2[(1, 0)] + 10.0).abs() < 0.0001);
}

#[test]
#[should_panic]
fn test_index01()
{
    let x = MatrixXf::ones(2, 1);
    assert!((x[(0, 1)] - 1.0).abs() < 0.0001);
}

#[test]
fn test_transposed01()
{
    let m1 = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let m2 = m1.t();
    
    assert!((m2[(0, 0)] - 1.0).abs() < 0.001);
    assert!((m2[(0, 1)] - 4.0).abs() < 0.001);
    assert!((m2[(1, 0)] - 2.0).abs() < 0.001);
    assert!((m2[(1, 1)] - 5.0).abs() < 0.001);
    assert!((m2[(2, 0)] - 3.0).abs() < 0.001);
    assert!((m2[(2, 1)] - 6.0).abs() < 0.001);
}

#[test]
fn test_mult_matrix01()
{
    let m1 = MatrixXf::from_vec(vec![-1.0, 2.0]).reshape(1, 2);
    let m2 = MatrixXf::from_vec(vec![ 1.0, 2.0]);
    let m3 = m1 * m2;

    assert!(m3.cols() == m3.rows() && m3.rows() == 1);
    assert!((m3[(0, 0)] - 3.0).abs() < 0.001);
}

#[test]
fn test_mult_matrix02()
{
    let m1 = MatrixXf::from_vec(vec![-1.0, 2.0]).reshape(1, 2);
    let m2 = MatrixXf::from_vec(vec![ 1.0, 2.0]);
    let m3 = &m1 * &m2;

    assert!(m3.cols() == m3.rows() && m3.rows() == 1);
    assert!((m3[(0, 0)] - 3.0).abs() < 0.001);
}

#[test]
fn test_mult_matrix03()
{
    let m1 = MatrixXf::from_vec(vec![-1.0, 2.0]);
    let m2 = MatrixXf::from_vec(vec![ 1.0, 2.0]);
    let m3 = m1.t() * m2;

    assert!(m3.cols() == m3.rows() && m3.rows() == 1);
    assert!((m3[(0, 0)] - 3.0).abs() < 0.001);
}

#[test]
fn test_mult_matrix04()
{
    let m1 = MatrixXf::from_vec(vec![-1.0, 2.0, 3.0]);
    let m2 = MatrixXf::from_vec(vec![ 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let m3 = m2 * m1;

    assert!(m3.cols() == 1);
    assert!(m3.rows() == 2);
    assert!((m3[(0, 0)] - 12.0).abs() < 0.001);
    assert!((m3[(1, 0)] - 24.0).abs() < 0.001);
}

#[test]
fn test_mult_matrix05()
{
    let m1 = MatrixXf::from_vec(vec![-1.0, 2.0, 3.0]);
    let m2 = MatrixXf::from_vec(vec![ 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    let m3 = (&m1.t() * &m2.t()).t();

    assert!(m3.cols() == 1);
    assert!(m3.rows() == 2);
    assert!((m3[(0, 0)] - 12.0).abs() < 0.001);
    assert!((m3[(1, 0)] - 24.0).abs() < 0.001);
}


#[test]
fn test_sum_matrix01()
{
    let m = MatrixXf::from_vec(vec![1.0, 2.0, 3.0]);
    assert!((m.sum() - 6.0).abs() < 0.0001);
}

#[test]
#[should_panic]
fn test_trace01()
{
    let m = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    let tr = m.trace();
    assert!((tr - 5.0).abs() < 0.0001);
}

#[test]
fn test_trace02()
{
    let m = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0]).reshape(2, 2);
    let tr = m.trace();
    assert!((tr - 5.0).abs() < 0.0001);
}

#[test]
fn test_abs01()
{
    let m = MatrixXf::from_vec(vec![1.0, -2.0, 3.0, -4.0]);
    let m2 = m.abs();

    assert!((m2[(0, 0)] - 1.0).abs() < 0.001);
    assert!((m2[(1, 0)] - 2.0).abs() < 0.001);
    assert!((m2[(2, 0)] - 3.0).abs() < 0.001);
    assert!((m2[(3, 0)] - 4.0).abs() < 0.001);
}

#[test]
fn test_diag01()
{
    let m = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0]).reshape(2, 2);
    let m2 = m.diag();
    let ans = MatrixXf::from_vec(vec![1.0, 4.0]);
    assert!((m2 - ans).abs().sum() < 0.0001);
}

#[test]
fn test_mat_macro01()
{
    let mat = mat![1.0, 2.0, 3.0; 4.0, 5.0, 6.0];
    let ans = MatrixXf::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).reshape(2, 3);
    assert!((mat - ans).abs().sum() < 0.00001);
}

}
