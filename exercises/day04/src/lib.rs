//! Day 4: functions, expressions, `if`, and loops.
//!
//!     cargo test -p day04
//!
//! Replace each `todo!()`. Don't change the tests at the bottom.

// Delete this line once every exercise is done.
#![allow(unused_variables)]

// ---------------------------------------------------------------------------
// Exercise 1: expressions and implicit return
// Write the body as ONE expression with no `return` and no semicolon.

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!()
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2: if / else if / else as an expression
// 80+ → 'A', 70–79 → 'B', 60–69 → 'C', 50–59 → 'D', below 50 → 'F'
// Try writing it as: `if ... { 'A' } else if ... { 'B' } ...` with no `return`.

pub fn grade(score: u32) -> char {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3: boolean logic
// A year is a leap year if it's divisible by 4,
// EXCEPT years divisible by 100, which are NOT leap years,
// EXCEPT years divisible by 400, which ARE leap years.
// 2024 → true, 1900 → false, 2000 → true, 2023 → false

pub fn is_leap_year(year: u32) -> bool {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4: FizzBuzz
// Multiple of 3 and 5 → "FizzBuzz", of 3 → "Fizz", of 5 → "Buzz", otherwise the number.
// To make a String: String::from("Fizz") and n.to_string()

pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5: `for` over a range
// Return 1 + 2 + ... + n. Use a `for` loop and a `mut` total (not the n*(n+1)/2 formula).

pub fn sum_to(n: u64) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 6: factorial with a `for` loop
// 0! = 1, 5! = 120. Use the range `1..=n`.

pub fn factorial(n: u32) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 7: `while` loop. The Collatz conjecture.
// Count the steps to reach 1: if n is even, n = n / 2, otherwise n = 3n + 1.
// collatz_steps(1) == 0, collatz_steps(6) == 8  (6,3,10,5,16,8,4,2,1)

pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 8: `for` over an array
// Return the largest number in the array.

pub fn largest(numbers: [i32; 6]) -> i32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 9: `loop` with `break value`
// Return the smallest perfect square (1, 4, 9, 16, ...) that is strictly GREATER than `limit`.
// Use `loop` and `break some_value;` so that the loop itself produces the result.

pub fn first_square_over(limit: u32) -> u32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 10: counting with a range and a condition
// How many numbers in from..=to are divisible by `by`?

pub fn count_divisible(from: u32, to: u32, by: u32) -> u32 {
    todo!()
}

// ===========================================================================
// Tests: don't edit below this line.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperature() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
    }

    #[test]
    fn grades() {
        assert_eq!(grade(95), 'A');
        assert_eq!(grade(80), 'A');
        assert_eq!(grade(79), 'B');
        assert_eq!(grade(65), 'C');
        assert_eq!(grade(50), 'D');
        assert_eq!(grade(49), 'F');
        assert_eq!(grade(0), 'F');
    }

    #[test]
    fn leap_years() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn fizz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(98), "98");
    }

    #[test]
    fn sums() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(10), 55);
        assert_eq!(sum_to(100_000), 5_000_050_000);
    }

    #[test]
    fn factorials() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    fn collatz() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn largest_number() {
        assert_eq!(largest([3, 9, 2, 7, 9, 1]), 9);
        assert_eq!(largest([-5, -2, -9, -3, -4, -8]), -2);
    }

    #[test]
    fn squares() {
        assert_eq!(first_square_over(0), 1);
        assert_eq!(first_square_over(10), 16);
        assert_eq!(first_square_over(16), 25);
    }

    #[test]
    fn divisible() {
        assert_eq!(count_divisible(1, 10, 3), 3);
        assert_eq!(count_divisible(1, 100, 7), 14);
        assert_eq!(count_divisible(5, 5, 5), 1);
    }
}
