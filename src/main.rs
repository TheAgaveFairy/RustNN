mod matrix;
use std::error::Error;
use crate::matrix::Matrix;
fn main() {
    let test0 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let test1 = test0.clone();


    println!("Creating a matrix with {} rows, {} cols, and the data {:?}.",2,3,test0);
    let test_matrix0 = Matrix::new(2, 3, test0).expect("matrix 0 error");
    println!("{}\n", test_matrix0);

    println!("Creating a matrix with {} rows, {} cols, and the data {:?}.",3,2,test1);
    let test_matrix1 = Matrix::new(3, 2, test1).expect("matrix 1 error");
    println!("{}\n", test_matrix1);

    println!("Calculating the dot product of these two matrices:");
    let result_matrix = test_matrix0.dot(&test_matrix1).expect("error with dot product");
    println!("{}\n", result_matrix);

    println!("Multiplying that result by the scalar 0.5:");
    let result_matrix = result_matrix.scale(0.5);
    println!("{}\n", result_matrix);

    println!("Showing the first row of that result:\n{:?}\n", result_matrix.get_row(0));
    println!("Showing the first col of that result:\n{:?}\n", result_matrix.get_col(0));

    let added_matrix = result_matrix.add_matrix(&result_matrix).expect("Adding didn't work.");
    println!("Adding this matrix to itself:\n{}\n", added_matrix);

    let transposed_matrix = test_matrix0.transpose();
    println!("Transposed version:\n{}\n", transposed_matrix);

    let added_scalar = transposed_matrix.add_scalar(-1.5);
    println!("Subtracting scalar 1.5:\n{}\n", added_scalar);

    let messed_up_matrix = added_scalar.map(&|v| v * 2.0 / 3.0);
    println!("Passing function:\n{}\n", messed_up_matrix);

    let test2 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let long_matrix = Matrix::new(2,4,test2).expect("");
    println!("Long matrix:\n{}\n",long_matrix);
    println!("Long matrix transposed:\n{}\n",long_matrix.transpose());

    let rand_matrix = Matrix::new_from_rand(3,4);
    println!("Random matrix:\n{}\n",rand_matrix);

    example().expect("TODO: panic message");
}
fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/mnist_test.csv").expect("Error reading csv.");
    let mut i = 0;
    for result in rdr.into_records() {
        if i == 5 {break}
        let record = result?;
        let actual: u8 = record[0].parse().expect("File error.");
        let mut pixels: Vec<u32> = Vec::new();
        for field in record.iter().skip(1) {
            pixels.push(field.parse::<u32>()?);
        }
        let sum: u32 = pixels.into_iter().sum();println!("This is a {}. {} pixel sum.", actual, sum);
        i += 1;
    }
    Ok(())
}