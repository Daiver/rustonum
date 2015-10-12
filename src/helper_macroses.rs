
#[macro_export]
macro_rules! mat {
    [ $($( $x: expr ),*);* ] => {{
        let mut tmp_vec = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        $(
            let mut inner_cols = 0;
            $(
                tmp_vec.push($x);
                inner_cols += 1;
            )*
            assert!(cols == 0 || inner_cols == cols);
            cols = inner_cols;
            rows += 1;
        )*
        assert!(rows > 0 && cols > 0);
        MatrixXf::construct(tmp_vec, rows, cols)
    }}
}

#[macro_export]
macro_rules! vec2 {
    [ $x: expr, $y: expr  ] => {{
        Vector2{values: [$x, $y]}
    }}
}

#[macro_export]
macro_rules! vec3 {
    [ $x: expr, $y: expr, $z: expr  ] => {{
        Vector3{values: [$x, $y, $z]}
    }}
}

#[macro_export]
macro_rules! vec2f {
    [ $x: expr, $y: expr  ] => {{
        Vector2f{values: [$x, $y]}
    }}
}

#[macro_export]
macro_rules! vec3f {
    [ $x: expr, $y: expr, $z: expr  ] => {{
        Vector3f{values: [$x, $y, $z]}
    }}
}

