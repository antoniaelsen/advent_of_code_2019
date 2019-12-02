
/*
 * Advent of Code 2019 - Day 1
 *
 * --- Part One ---
 * The Elves quickly load you into a spacecraft and prepare to launch.
 *
 * At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of
 *  fuel required yet.
 * Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module,
 *  take its mass, divide by three, round down, and subtract 2.
 *
 * For example:
 *
 * For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
 * For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
 * For a mass of 1969, the fuel required is 654.
 * For a mass of 100756, the fuel required is 33583.
 * The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed
 *  for the mass of each module (your puzzle input), then add together all the fuel values.
 *
 * What is the sum of the fuel requirements for all of the modules on your spacecraft?
 *
 * To begin, get your puzzle input [see input.txt].
 *
 * --- Part Two ---
 * During the second Go / No Go poll, the Elf in charge of the Rocket Equation Double-Checker stops the launch sequence.
 * Apparently, you forgot to include additional fuel for the fuel you just added.
 *
 * Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2. However,
 *  that fuel also requires fuel, and that fuel requires fuel, and so on. Any mass that would require negative fuel
 *  should instead be treated as if it requires zero fuel; the remaining mass, if any, is instead handled by wishing
 *  really hard, which has no mass and is outside the scope of this calculation.
 *
 * So, for each module mass, calculate its fuel and add it to the total.
 * Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel
 *  requirement is zero or negative. For example:
 *
 * A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which
 *  would call for a negative fuel), so the total fuel required is still just 2.
 * At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then
 *  requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total
 *  fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
 * The fuel required by a module of mass 100756 and its fuel is:
 *  33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
 *
 * What is the sum of the fuel requirements for all of the modules on your spacecraft when also taking into account the
 *  mass of the added fuel? (Calculate the fuel requirements for each module separately, then add them all up at the end.)
 */

/*
 * Learning Rust
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

fn parse_inputs(path: impl AsRef<Path>) -> Vec<i32> {
  strings_to_ints(lines_from_file(path))
}

fn calculate_fuel(mass: i32) -> i32 {
  mass / 3 - 2
}

fn calculate_fuel_recursive(fuel_mass: i32) -> i32 {
  let recursive_fuel_mass = calculate_fuel(fuel_mass);
  if recursive_fuel_mass < 0 { return fuel_mass };
  fuel_mass + calculate_fuel_recursive(recursive_fuel_mass)
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
  let inputs_filename = "input.txt";
  let masses = parse_inputs(inputs_filename);

  // --- Part One ---
  let test_case_inputs = vec![12, 14, 1969, 100756];
  let test_case_results = vec![2, 2, 654, 33583];

  println!("Calculating total test case fuel requirements (non-recursive)...");
  for it in test_case_inputs.iter().zip(test_case_results.iter()) {
    let (input, result) = it;
    let calc = calculate_fuel(*input);
    let pass = calc - result == 0;
    // Learning Rust: ternary
    println!("- {} - For a mass of {}, got {} / {}", (if pass { "PASS" } else { "FAIL "}), input, calc, *result);
  }
  println!();

  let fuel_per_mass: Vec<i32> = masses.iter().map(|it| calculate_fuel(*it)).collect();
  let fuel_total: i32 = fuel_per_mass.iter().sum();
  println!("Calculated fuel total (non-recursive): {}\n", fuel_total);


  // --- Part Two ---
  let test_case_results_recursive = vec![2, 2, 966, 50346];
  println!("Calculating total test case fuel requirements (recursive)...");
  for it in test_case_inputs.iter().zip(test_case_results_recursive.iter()) {
    let (input, result) = it;
    let calc = calculate_fuel_recursive(calculate_fuel(*input));
    let pass = calc - result == 0;
    // Learning Rust: ternary
    println!("- {} - For a mass of {}, got {} / {}", (if pass { "PASS" } else { "FAIL "}), input, calc, *result);
  }
  println!();

  let fuel_per_mass_recursive: Vec<i32> = masses.iter().map(|it| calculate_fuel_recursive(calculate_fuel(*it))).collect();
  let fuel_total_recursive: i32 = fuel_per_mass_recursive.iter().sum();
  println!("Calculated fuel total (recursive): {}", fuel_total_recursive);
}