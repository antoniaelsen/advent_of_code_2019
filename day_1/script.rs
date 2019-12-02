
/*
 * Advent of Code 2019 - Day 1
 *
 * Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
 * 
 * For example:
 * 
 * For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
 * For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
 * For a mass of 1969, the fuel required is 654.
 * For a mass of 100756, the fuel required is 33583.
 * The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.
 * 
 * What is the sum of the fuel requirements for all of the modules on your spacecraft?
 * 
 * To begin, get your puzzle input [see input.txt].
 */

/*
 * Learning Rust
 * 
 * The following solution to the AoC problem is relatively verbose, for clarity when learning new material.
 * 
 * 
 * Printing - https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
 * Comments - https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html
 * Functions - https://doc.rust-lang.org/stable/rust-by-example/fn.html
 * Closures - https://doc.rust-lang.org/rust-by-example/fn/closures.html
 * File I/O - https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html
 * Iterators - https://doc.rust-lang.org/std/iter/trait.Iterator.html
 * Collections - https://doc.rust-lang.org/std/collections/index.html
 * Result - https://doc.rust-lang.org/std/result/
 * Option and Unwrap - https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap.html
 */

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn lines_from_file(path: impl AsRef<Path>) -> Vec<String> {
  // Learning Rust: match is an expression
  let file = match File::open(path) {
      Err(why) => panic!("Failed to open file: {}", why.description()),
      Ok(file) => file,
  };

  let reader = BufReader::new(file);

  // Learning Rust: lines returns an iterator over all lines; collect turns the iter into a collection
  // Learning Rust: 'Result' must be handled - unwrap, expect, match or ?
  // Learning Rust: return by omitting ';'
  reader.lines()
    .map(|line| line.expect("Could not parse line"))
    .collect()
}

fn strings_to_ints(strings: Vec<String>) -> Vec<i32> {
  // Learning Rust: turbofish syntax to help determine type
  strings.iter().map(|string| string.parse::<i32>().unwrap()).collect()
}

/*
 * Main - 
 * Load input file
 * Parse input file - one module mass per line
 * For each module mass, calculate fuel
 *  fuel per module = floor(module mass / 3) - 2
 * Sum all calculated fuel
 */
fn main() {
  let calculate_fuel = |mass: &i32| { (*mass as f64 / 3 as f64).floor() as i32 - 2 };
  let inputs_filename = "input.txt";
  let masses = strings_to_ints(lines_from_file(inputs_filename));
  let fuel_per_mass: Vec<i32> = masses.iter().map(calculate_fuel).collect();
  let fuel_total: i32 = fuel_per_mass.iter().sum();

  let test_case_inputs = vec![12, 14, 1969, 100756];
  let test_case_results = vec![2, 2, 654, 33583];

  println!("Calculating total test case fuel requirements...");
  for it in test_case_inputs.iter().zip(test_case_results.iter()) {
    let (input, result) = it;
    let calc = calculate_fuel(input);
    let pass = calc - result == 0;
    println!("- {} - For a mass of {}, got {} / {}", (if pass { "PASS" } else { "FAIL "}), input, calc, *result);
  }

  println!("Calculating total fuel requirements...");
  // println!("- Got masses: {:?}", masses);
  // println!("- Got fuel per mass: {:?}", fuel_per_mass);
  println!("- Calculated fuel total: {:?}", fuel_total);

  // println!("Calculated fuel for all modules...");
  // println!("Calculated fuel for module {0} with mass {1} - {2}");
}