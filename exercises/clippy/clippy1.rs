// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

use std::f64::EPSILON;

use std::f32;

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    if (y -  x).abs() > EPSILON {
        println!("Success!");
    }
}
