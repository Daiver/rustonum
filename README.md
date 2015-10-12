# rustonum - primitive, dirty and inefficient linear algebra for rust 

Just for fun project for educational purposes

Use it at your own risk

Install
--------
Cargo.toml
```
[dependencies.rustonum]
git = "https://github.com/daiver/rustonum"
# or 
#path = "path/to/rustonum"
```

Examples
--------

Matrix operations example 

from `examples/common.rs`
   
```rust
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
        println!("{}", mat5);
        let mat6 = mat![-30.0, 50.0; -77.0, -100.0]; //Fill matrix in matlab way
        let mat7 = mat6 + mat5 / 2.0;
        println!("{}", mat7.sum());
    }
``` 

Vector operations example

from `examples/vector.rs`

```rust
    #[macro_use] extern crate rustonum;

    use rustonum::vector::*;

    fn main()
    {
        let v1 = vec3f![1.0, 2.0, 3.0];
        let v2 = vec3![5.0, 6.0, 7.0];
        let v3 = v1.cross(v2);

        println!("v3 = {:?}", v3);
        println!("length of v3 = {}", v3.length());
        println!("direction of v3 = {:?}", v3.normalized());
        println!("restored v3 = {:?}", v3.normalized() * v3.length());

        let v4 = vec3f![2.0, 3.0, 1.0];
        let v5 = vec3![5.0, 7.0, 3.0];
        let v6 = vec3![1.0, 1.0, 1.0];
        let v7 = 2.0 * (v4 + v6 * 0.5) - v5;
        println!("v7 = {}", v7.sum());
        let v8 = 0.1 * vec2![-1.0, 0.0] * vec2![1.0, 2.0];
        println!("v8 = {:?}", v8);
    }

```

See `examples/*` and `tests/*` for more examples

Pull requests are welcomed

Inspiration
---------
https://github.com/SiegeLord/RustAlgebloat

https://github.com/hawkw/lin

Dependences
------------
Some numerical traits 
https://github.com/rust-lang-deprecated/num
