// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

use std::f32;

fn main() {
    // clippy::approx_constant rejects a hand-written 3.14 because it only
    // pretends to be pi; std::f32::consts carries the full-precision value.
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    // powi is an inherent method on f32, so calling it through the type path
    // trips clippy::unnecessary_operation-style complaints; use method syntax.
    let area = pi * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
