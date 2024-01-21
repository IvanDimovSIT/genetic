use std::io;

use genetic::{ErrorCalculator, optimise};

mod genetic;

// solve:
// x^2 + y*3 - 2.5 = 0
struct MyErrorCalculator;
impl ErrorCalculator for MyErrorCalculator {
    fn calculate(&self, genes: &Vec<f64>) -> f64 {
        (genes[0]*genes[0]+genes[1]*3.0 - 2.5).abs()
    }
}

fn main() {
    println!("Enter mutation chance:");
    let mut mutation_chance_str = "".to_string();
    io::stdin().read_line(&mut mutation_chance_str).expect("Error reading input");
    let mutation_chance: f64 = mutation_chance_str.trim().parse().expect("Invalid float");

    println!("Enter epochs:");
    let mut epochs_str = "".to_string();
    io::stdin().read_line(&mut epochs_str).expect("Error reading input");
    let epochs: usize = epochs_str.trim().parse().expect("Invalid value");

    let error_calc = MyErrorCalculator;
    let solution = optimise(2, &error_calc, mutation_chance, epochs);

    println!("Solution: x={} y={}", solution[0], solution[1]);
    println!("Error: {}", error_calc.calculate(&solution));
}
