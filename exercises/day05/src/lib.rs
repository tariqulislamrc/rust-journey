//! Day 5: practice day. Classic number problems with everything from Days 1–4.
//!
//!     cargo test -p day05
//!     cargo test -p day05 --release   # try this after Exercise 1. Notice the speed difference!

// Delete this line once every exercise is done.
#![allow(unused_variables)]

// ---------------------------------------------------------------------------
// Exercise 1: Fibonacci, recursive
// fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2)
// Write it as a recursive function that calls itself.

pub fn fib_recursive(n: u32) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2: Fibonacci, iterative
// Same sequence, but with a loop and two variables (no recursion).
// Returns u128 because fib(150) is a 31-digit number!

pub fn fib_iterative(n: u32) -> u128 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3: primes
// A prime is greater than 1 and divisible only by 1 and itself.
// You only need to check divisors up to the square root: loop while d * d <= n.

pub fn is_prime(n: u64) -> bool {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4: greatest common divisor (Euclid's algorithm)
// While b != 0: (a, b) = (b, a % b). The answer is a.
// gcd(48, 18) == 6

pub fn gcd(a: u64, b: u64) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5: digit sum
// 1234 → 1 + 2 + 3 + 4 = 10. Use % 10 to get the last digit and / 10 to drop it.

pub fn digit_sum(n: u64) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 6: reverse a number
// 1234 → 4321, 1200 → 21 (leading zeros disappear), 7 → 7

pub fn reverse_number(n: u64) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 7: is it a palindrome number?
// 12321 → true, 1221 → true, 123 → false. Reuse reverse_number!

pub fn is_palindrome_number(n: u64) -> bool {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 8: formatted multiplication row
// Build a String with n×1 to n×upto, each number right-aligned in width 4.
// multiplication_row(3, 5) == "   3   6   9  12  15"
//
// Start with `let mut row = String::new();`
// and append pieces with `row.push_str(&format!("{:>4}", value));`
// (`format!` works like `println!` but returns a String instead of printing it.)

pub fn multiplication_row(n: u32, upto: u32) -> String {
    todo!()
}

// ===========================================================================
// Tests: don't edit below this line.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_recursive() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(10), 55);
        assert_eq!(fib_recursive(30), 832_040);
    }

    #[test]
    fn fibonacci_iterative() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
        assert_eq!(fib_iterative(10), 55);
        assert_eq!(fib_iterative(90), 2_880_067_194_370_816_120);
        assert_eq!(fib_iterative(150), 9_969_216_677_189_303_386_214_405_760_200);
    }

    #[test]
    fn primes() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(97));
        assert!(!is_prime(91)); // 7 × 13
        assert!(is_prime(1_000_000_007));
    }

    #[test]
    fn greatest_common_divisor() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(100, 0), 100);
    }

    #[test]
    fn digits() {
        assert_eq!(digit_sum(0), 0);
        assert_eq!(digit_sum(1234), 10);
        assert_eq!(digit_sum(99_999), 45);
    }

    #[test]
    fn reversing() {
        assert_eq!(reverse_number(1234), 4321);
        assert_eq!(reverse_number(1200), 21);
        assert_eq!(reverse_number(7), 7);
        assert_eq!(reverse_number(0), 0);
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome_number(12321));
        assert!(is_palindrome_number(1221));
        assert!(is_palindrome_number(5));
        assert!(!is_palindrome_number(123));
        assert!(!is_palindrome_number(10));
    }

    #[test]
    fn table_row() {
        assert_eq!(multiplication_row(3, 5), "   3   6   9  12  15");
        assert_eq!(multiplication_row(12, 3), "  12  24  36");
    }
}
