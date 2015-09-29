//#[macro_use]

use std;
use std::ops::{Index, IndexMut, Add, Sub, Mul};

#[derive(Debug, Clone)]
pub struct MatrixXf
{
    values       : Vec<f32>,
    rows         : usize,
    cols         : usize,
}

impl MatrixXf {

    pub fn zeros(rows: usize, cols: usize) -> MatrixXf
    {
        let values = std::iter::repeat(0.0).take(rows * cols).collect::<Vec<_>>();
        MatrixXf {values: values, rows: rows, cols: cols}
    }


    pub fn ones(rows: usize, cols: usize) -> MatrixXf
    {
        let values = std::iter::repeat(1.0).take(rows * cols).collect::<Vec<_>>();
        MatrixXf {values: values, rows: rows, cols: cols}
    }

    pub fn consts(rows: usize, cols: usize, value: f32) -> MatrixXf
    {
        MatrixXf::ones(rows, cols) * value
    }

    pub fn ident(rows: usize) -> MatrixXf
    {
        let mut res = MatrixXf::zeros(rows, rows);
        for i in (0..rows){
            res[(i, i)] = 1.0;
        }
        res
    }

    pub fn from_vec(vec: Vec<f32>) -> MatrixXf
    {
        let rows = vec.len();
        MatrixXf {
            values: vec,
            rows: rows,
            cols: 1
        }
    }
    
    pub fn reshape(self, rows: usize, cols: usize) -> MatrixXf 
    {
        assert!(rows * cols == self.rows * self.cols);
        MatrixXf {
            values: self.values,
            rows: rows,
            cols: cols
        }
    }

    pub fn rows(&self) -> usize
    {
        self.rows
    }

    pub fn cols(&self) -> usize
    {
        self.cols
    }

    pub fn values(&self) -> &Vec<f32>
    {
        &self.values
    }

    pub fn is_square(&self) -> bool
    {
        self.cols == self.rows
    }

    pub fn size(&self) -> usize
    {
        self.values.len()
    }

    pub fn max(&self) -> f32
    {
        assert!(self.values.len() > 0);
        self.values.iter().cloned().fold(self.values[0], f32::max)
    }

    pub fn min(&self) -> f32
    {
        assert!(self.values.len() > 0);
        self.values.iter().cloned().fold(self.values[0], f32::min)
    }

    //produced copy!/not optimal
    pub fn transposed(&self) -> MatrixXf
    {
        let mut res = MatrixXf::zeros(self.cols(), self.rows());
        for i in (0..self.rows()){
            for j in (0..self.cols()){
                res[(j, i)] = self[(i, j)]
            }
        }
        res
    }

    pub fn t(&self) -> MatrixXf
    {
        self.transposed()
    }

    pub fn sum(&self) -> f32 
    {
        assert!(self.values.len() > 0);
        self.values.iter().cloned().fold(0.0, f32::add)
    }

    pub fn abs(&self) -> MatrixXf
    {
        MatrixXf{
            rows: self.rows, 
            cols: self.cols, 
            values: self.values.iter().cloned().map(f32::abs).collect::<Vec<_>>()
        }
    }

    pub fn trace(&self) -> f32
    {
        assert!(self.is_square());
        let mut res = 0.0;
        for i in (0..self.rows){
            res += self[(i, i)];
        }
        res
    }

    pub fn diag(&self) -> MatrixXf
    {
        assert!(self.is_square());
        let mut res = MatrixXf::zeros(self.rows, 1);
        for i in (0..self.rows){
            res[(i, 0)] = self[(i, i)];
        }
        res
    }
}

impl Add for MatrixXf {
    type Output = MatrixXf;

    fn add(self, _rhs: MatrixXf) -> MatrixXf {
        assert!(self.rows == _rhs.rows);
        assert!(self.cols == _rhs.cols);
        let mut res = MatrixXf {
            values : self.values.clone(), rows : self.rows, cols : self.cols
        };
        for i in 0 .. (res.values.len()) {
            res.values[i] +=  _rhs.values[i];
        }
        res
    }
}


impl<'a, 'b> Add<&'b MatrixXf> for &'a MatrixXf {
    type Output = MatrixXf;

    fn add(self, _rhs: &'b MatrixXf) -> MatrixXf {
        assert!(self.rows == _rhs.rows);
        assert!(self.cols == _rhs.cols);
        let mut res = MatrixXf {
            values : self.values.clone(), rows : self.rows, cols : self.cols
        };
        for i in 0 .. (res.values.len()) {
            res.values[i] +=  _rhs.values[i];
        }
        res
    }
}

impl Add<f32> for MatrixXf {
    type Output = MatrixXf;

    fn add(self, _rhs: f32) -> MatrixXf {
        add_matrix_by_const(&self, _rhs)
    }
}

impl Add<MatrixXf> for f32 {
    type Output = MatrixXf;

    fn add(self, _rhs: MatrixXf) -> MatrixXf {
        add_matrix_by_const(&_rhs, self)
    }
}


impl<'a> Add<f32> for &'a MatrixXf {
    type Output = MatrixXf;

    fn add(self, _rhs: f32) -> MatrixXf {
        add_matrix_by_const(self, _rhs)
    }
}

impl<'a> Add<&'a MatrixXf> for f32 {
    type Output = MatrixXf;

    fn add(self, _rhs: &'a MatrixXf) -> MatrixXf {
        add_matrix_by_const(_rhs, self)
    }
}

impl Sub for MatrixXf {
    type Output = MatrixXf;

    fn sub(self, _rhs: MatrixXf) -> MatrixXf {
        assert!(self.rows == _rhs.rows);
        assert!(self.cols == _rhs.cols);
        let mut res = MatrixXf {
            values : self.values.clone(), rows : self.rows, cols : self.cols
        };
        for i in 0 .. (res.values.len()) {
            res.values[i] -=  _rhs.values[i];
        }
        res
    }
}

impl<'a, 'b> Sub<&'b MatrixXf> for &'a MatrixXf {
    type Output = MatrixXf;

    fn sub(self, _rhs: &'b MatrixXf) -> MatrixXf {
        assert!(self.rows == _rhs.rows);
        assert!(self.cols == _rhs.cols);
        let mut res = MatrixXf {
            values : self.values.clone(), rows : self.rows, cols : self.cols
        };
        for i in 0 .. (res.values.len()) {
            res.values[i] -=  _rhs.values[i];
        }
        res
    }
}

impl Sub<f32> for MatrixXf {
    type Output = MatrixXf;

    fn sub(self, _rhs: f32) -> MatrixXf {
        add_matrix_by_const(&self, -_rhs)
    }
}

impl Sub<MatrixXf> for f32 {
    type Output = MatrixXf;

    fn sub(self, _rhs: MatrixXf) -> MatrixXf {
        sub_matrix_from_const(&_rhs, self)
    }
}


impl<'a> Sub<f32> for &'a MatrixXf {
    type Output = MatrixXf;

    fn sub(self, _rhs: f32) -> MatrixXf {
        sub_matrix_from_const(self, _rhs)
    }
}

impl<'a> Sub<&'a MatrixXf> for f32 {
    type Output = MatrixXf;

    fn sub(self, _rhs: &'a MatrixXf) -> MatrixXf {
        sub_matrix_from_const(_rhs, self)
    }
}

impl Mul for MatrixXf {
    type Output = MatrixXf;

    fn mul(self, _rhs: MatrixXf) -> MatrixXf {
        mult_matrix_by_matrix(&self, &_rhs)
    }
}

impl<'a, 'b> Mul<&'b MatrixXf> for &'a MatrixXf {
    type Output = MatrixXf;

    fn mul(self, _rhs: &'b MatrixXf) -> MatrixXf {
        mult_matrix_by_matrix(self, _rhs)
    }
}


impl Mul<f32> for MatrixXf {
    type Output = MatrixXf;

    fn mul(self, _rhs: f32) -> MatrixXf {
        mult_matrix_by_const(&self, _rhs)
    }
}

impl Mul<MatrixXf> for f32 {
    type Output = MatrixXf;

    fn mul(self, _rhs: MatrixXf) -> MatrixXf {
        mult_matrix_by_const(&_rhs, self)
    }
}


impl Index<(usize, usize)> for MatrixXf {
    type Output = f32;
    fn index<'a>(&'a self, _index : (usize, usize)) -> &'a f32 {
        let (i, j) = _index;
        assert!(i < self.rows);
        assert!(j < self.cols);
        &(self.values[i*self.cols + j])
    }
}

impl IndexMut<(usize, usize)> for MatrixXf {
    fn index_mut<'a>(&'a mut self, _index : (usize, usize)) -> &'a mut f32 {
        let (i, j) = _index;
        assert!(i < self.rows);
        assert!(j < self.cols);
        & mut (self.values[i*self.cols + j])
    }
}


fn mult_matrix_by_matrix(mat1: &MatrixXf, mat2: &MatrixXf) -> MatrixXf
{
    let n = mat1.rows;
    let k = mat1.cols;
    let m = mat2.cols;
    assert!(k == mat2.rows);
    let mut res = MatrixXf::zeros(n, m);
    for i in (0..n){
        for j in (0..m){
            for p in (0..k){
                res[(i, j)] += mat1[(i, p)] * mat2[(p, j)];
            }
        }
    }
    res
}

fn mult_matrix_by_const(mat: &MatrixXf, _rhs: f32) -> MatrixXf
{
    MatrixXf {
        values : mat.values.iter().cloned().map(|x| x * _rhs).collect::<Vec<_>>(), 
        rows : mat.rows, cols : mat.cols
    }
}

fn add_matrix_by_const(mat: &MatrixXf, _rhs: f32) -> MatrixXf
{
    MatrixXf {
        values : mat.values.iter().cloned().map(|x| x + _rhs).collect::<Vec<_>>(), 
        rows : mat.rows, cols : mat.cols
    }
}

fn sub_matrix_from_const(mat: &MatrixXf, _rhs: f32) -> MatrixXf
{
    MatrixXf {
        values : mat.values.iter().cloned().map(|x| _rhs - x).collect::<Vec<_>>(), 
        rows : mat.rows, cols : mat.cols
    }
}

#[macro_export]
macro_rules! mat {
    [ $($( $x: expr ),*);* ] => {{
        let mut tmp_vec = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        let mut is_first_row_collected = false;
        $(
            let mut inner_cols = 0;
            $(
                tmp_vec.push($x);
                inner_cols += 1;
            )*
            if is_first_row_collected {
                assert!(inner_cols == cols);
            } else {
                is_first_row_collected = true;
                cols = inner_cols;
            }
            rows += 1;
        )*
        MatrixXf::from_vec(tmp_vec).reshape(rows, cols)
    }}
}

