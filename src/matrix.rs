use std::fmt;
use rand::{thread_rng, Rng};
#[derive(Debug)]
    pub(crate) struct Matrix {
        pub num_rows: usize,
        pub num_cols: usize,
        data: Vec<f64>,
    }

    impl Matrix {
        pub(crate) fn new(num_rows: usize, num_cols: usize, data: Vec<f64>) -> Result<Self, &'static str> {
            let expected_length = num_rows * num_cols;
            if data.len() != expected_length {
                return Err("Data length does not match matrix dimensions");
            }

            Ok(Matrix {
                num_rows,
                num_cols,
                data,
            })
        }
        pub(crate) fn new_from_rand(num_rows: usize, num_cols: usize) -> Self {
            let length = num_rows * num_cols;
            let mut data: Vec<f64> = Vec::with_capacity(length);
            let mut rng = thread_rng();
            for _ in 0..length {
                data.push(rng.gen::<f64>() * 2.0 - 1.0);
            }
            Matrix{
                num_rows,
                num_cols,
                data
            }
        }
        fn calc_index(&self, row: usize, col: usize) -> usize {
            row * self.num_cols + col
        }
        // fn calc_new_index(row_wanted: usize, col_wanted: usize, num_rows: usize, num_cols: usize) -> usize {
        //     println!("calc new index. want {}, {} in a matrix of size {}, {}", row_wanted, col_wanted, num_rows, num_cols);
        //     col_wanted * num_rows + row_wanted
        // }
        fn get_shape(&self) -> (usize, usize) {
            (self.num_rows, self.num_cols)
        }
        fn length(&self) -> usize {
            self.num_rows * self.num_cols
        }
        fn get(&self, row: usize, col: usize) -> &f64 {
            &self.data[self.calc_index(row, col)]
        }
        pub(crate) fn get_row(&self, row: usize) -> Vec<f64> {
            let start = self.calc_index(row, 0);
            let end = self.calc_index(row + 1, 0);
            self.data[start..end].to_vec()
        }
        pub(crate) fn get_col(&self, col: usize) -> Vec<f64> {
            let mut our_col: Vec<f64> = Vec::with_capacity(self.num_rows);
            for i in 0..self.num_rows {
                our_col.push(*self.get(i, col));
            }
            our_col
        }
        pub(crate) fn dot(&self, other: &Matrix) -> Result<Matrix, &'static str> {
            if self.num_cols != other.num_rows {
                return Err("Matrix shapes are incompatible for a dot product.");
            }

            let result_rows = self.num_rows;
            let result_cols = other.num_cols;
            let mut result_data = Vec::with_capacity(result_rows * result_cols);

            for i in 0..result_rows {
                for j in 0..result_cols {
                    let mut sum = 0.0;
                    let our_row = self.get_row(i);
                    let our_col = other.get_col(j);
                    for k in 0..self.num_cols {
                        sum += our_row[k] * our_col[k];
                    }
                    result_data.push(sum);
                }
            }

            Ok(Matrix {
                num_rows: result_rows,
                num_cols: result_cols,
                data: result_data,
            })
        }
        pub(crate) fn scale(&self, scalar: f64) -> Self {
            let mut new_data: Vec<f64> = self.data.clone();
            for i in &mut new_data {
                *i *= scalar;
            }
            Matrix {
                num_rows: self.num_rows,
                num_cols: self.num_cols,
                data: new_data,
            }
        }
        pub(crate) fn add_scalar(&self, scalar: f64) -> Self {
            let mut new_data = Vec::with_capacity(self.length());
            for v in &self.data {
                new_data.push(v + scalar);
            }
            Matrix {
                num_rows: self.num_rows,
                num_cols: self.num_cols,
                data: new_data,
            }
        }
        pub(crate) fn add_matrix(&self, other: &Matrix) -> Result<Matrix, &'static str> {
            if self.get_shape() != other.get_shape() {
                return Err("Incompatible shapes for addition.")
            }
            let mut new_data: Vec<f64> = Vec::with_capacity(self.length());
            for i in 0..self.length() {
                let sum = self.data[i] + other.data[i];
                new_data.push(sum);
            }
            Ok(Matrix {
                num_rows: self.num_rows,
                num_cols: self.num_cols,
                data: new_data,
            })
        }
        pub(crate) fn transpose(&self) -> Self {
            let mut new_data: Vec<f64> = Vec::with_capacity(self.length());
            let new_rows = self.num_cols;
            let new_cols = self.num_rows;
            for r in 0..new_rows {
                for c in 0..new_cols {
                    let temp_i = c * new_rows + r;
                    //println!("transpose ti: {}", Self::calc_new_index(r,c,new_rows, new_cols));
                    new_data.push(self.data[temp_i].clone());
                }
            }

            Matrix {
                num_rows: self.num_cols,
                num_cols: self.num_rows,
                data: new_data,
            }
        }
        pub(crate) fn map(&self, function: &dyn Fn(f64) -> f64) -> Self {
            let new_data: Vec<f64> = self.data.clone().into_iter().map(
                |val| function(val)).collect();

            Matrix {
                num_rows: self.num_cols,
                num_cols: self.num_rows,
                data: new_data,
            }

        }
    }

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut display = "[".to_string();
            let (num_rows, num_cols) = self.get_shape();

            for row in 0..num_rows {
                if row > 0 { display.push(' '); }
                display.push('[');
                for col in 0..num_cols {
                    let val = self.get(row, col);
                    let mut temp = format!("{:.2}", val);
                    if col < num_cols - 1 { temp.push_str(", "); }
                    display.push_str(temp.as_str());
                }
                display.push(']');
                if row < num_rows - 1 { display.push('\n'); }
            }
            display.push(']');
            write!(f, "{}", display)
        }
    } // to_string()