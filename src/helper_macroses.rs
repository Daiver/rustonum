//use super::MatrixXf;

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

