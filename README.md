# rustonum - primitive, dirty and inefficient linear algebra for rust 

Just for fun project for educational purposes

Use it at your own risk

Examples
--------
from `examples/common.rs`
   
    #[macro_use] extern crate rustonum;

    use rustonum::MatrixXf;

    fn main()
    {
        let mat1 = MatrixXf::from_vec(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
            //create row vector from Vec<f32>
        let mat2 = mat1.reshape(2, 3); //Create matrix 2 x 3
        let mat3 = MatrixXf::ones(2, 3); //Create matrix 2 x 3 and fill it by ones
        let mat4 = &mat2 + &(3.0 * mat3); //component-wise summation and multiplication 
        let mat5 = mat2 * mat4.t(); //matrix multiplication
        println!("{:?}", mat5);
        let mat6 = mat![-30.0, 50.0; -77.0, -100.0]; //Fill matrix in matlab way
        let mat7 = mat6 + mat5 / 2.0;
        println!("{}", mat7.sum());
    }
 

Watch `examples` and `tests` for more examples

Pull requests are welcomed

Inspiration
---------
https://github.com/SiegeLord/RustAlgebloat
https://github.com/hawkw/lin
