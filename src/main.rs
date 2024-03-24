mod matrix;

//mod matrix;
//use crate::matrix;
fn main() {
    let test0 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let test1 = test0.clone();


    println!("Creating a matrix with {} rows, {} cols, and the data {:?}.",2,3,test0);
    let test_matrix0 = matrix::Matrix::new(2, 3, test0).expect("matrix 0 error");
    println!("{}\n", test_matrix0.to_string());

    println!("Creating a matrix with {} rows, {} cols, and the data {:?}.",3,2,test1);
    let test_matrix1 = matrix::new(3, 2, test1).expect("matrix 1 error");
    println!("{}\n", test_matrix1.to_string());

    println!("Calculating the dot product of these two matrices:");
    let result_matrix = test_matrix0.dot(&test_matrix1).expect("error with dot product");
    println!("{}\n", result_matrix.to_string());

    println!("Multiplying that result by the scalar 0.5:");
    let result_matrix = result_matrix.scale(0.5);
    println!("{}\n", result_matrix.to_string());

    println!("Showing the first row of that result:\n{:?}\n", result_matrix.get_row(0));
    println!("Showing the first col of that result:\n{:?}\n", result_matrix.get_col(0));

    let added_matrix = result_matrix.add_matrix(&result_matrix).expect("Adding didn't work.");
    println!("Adding this matrix to itself:\n{}\n", added_matrix.to_string());

    let transposed_matrix = added_matrix.transpose();
    println!("Transposed version:\n{}\n", transposed_matrix.to_string());

    let added_scalar = transposed_matrix.add_scalar(-9.5);
    println!("Subtracting scalar 9.5:\n{}\n", added_scalar.to_string());
}