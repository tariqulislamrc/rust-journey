// Day 3: variables, mutability, constants, shadowing and data types.
//
// Replace each `todo!()` with a real implementation, then run:
//
//     cargo test -p day03
//
// All tests green = exercise done. Don't change the tests at the bottom.

// Unimplemented functions don't use their parameters yet, which would cause warnings.
// Delete this line once every exercise is done.


// ---------------------------------------------------------------------------
// Exercise 1: constants
// Define a constant named SECONDS_PER_DAY (type u32) right here, above the function.
// Then use it to return how many seconds are in `days` days.
const SECONDS_PER_DAY : u32 = 24 * 60 * 60;
pub fn seconds_in_days(days: u32) -> u32 {
    SECONDS_PER_DAY * days
}

// ---------------------------------------------------------------------------
// Exercise 2: mutability
// Start with `let total = price;` and change it step by step:
//   1. add `tax` to it
//   2. subtract `discount` from it
// then return it. (You'll need `mut`. Use `+=` and `-=`.)

pub fn final_price(price: u32, tax: u32, discount: u32) -> u32 {
    let mut total = price;
    total += tax;
    total -= discount;
    total
}

// ---------------------------------------------------------------------------
// Exercise 3: shadowing
// `input` is text like "  21 ". Using SHADOWING (re-declaring `input` with `let`):
//   1. let input = trimmed input
//   2. let input: i64 = parsed input (use .expect("not a number"))
// then return input * 2.

pub fn parse_and_double(input: &str) -> i64 {
    let input = input.trim();
    let input : i64 = input.parse().expect("not a number");
    input * 2
}

// ---------------------------------------------------------------------------
// Exercise 4: integer overflow
// u8 holds 0..=255. Return a + b, but:
//   - add_saturating: stop at the maximum (250 + 10 = 255)
//   - add_wrapping: wrap around (250 + 10 = 4)
//   - add_checked_or_zero: if it overflows, return 0
// Look for the methods saturating_add, wrapping_add and checked_add in the lesson.

pub fn add_saturating(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}

pub fn add_wrapping(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

pub fn add_checked_or_zero(a: u8, b: u8) -> u8 {
    a.checked_add(b).unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Exercise 5: integer division and tuples
// Split a restaurant bill (in taka) between `people`.
// Return a tuple: (amount each person pays, taka left over).
// Example: split_bill(1000, 3) == (333, 1)

pub fn split_bill(total: u32, people: u32) -> (u32, u32) {
    (total/ people, total%people)
}

// ---------------------------------------------------------------------------
// Exercise 6: arrays and floats
// Return the average of the four numbers.

pub fn average(nums: [f64; 4]) -> f64 {
    nums.iter().sum::<f64>() / nums.len() as f64
}

// ---------------------------------------------------------------------------
// Exercise 7: casting with `as`
// Return what percentage `part` is of `whole`, as an f64.
// Example: percentage(1, 4) == 25.0. Careful: 1 / 4 in integers is 0!

pub fn percentage(part: u32, whole: u32) -> f64 {
    part as f64 / whole as f64 * 100.0
}

// ---------------------------------------------------------------------------
// Exercise 8: char
// Return how many BYTES this character takes in UTF-8.
// 'a' takes 1, 'অ' (Bangla letter) takes 3, '🦀' takes 4.

pub fn utf8_bytes(c: char) -> usize {
    c.len_utf8()
}

// ---------------------------------------------------------------------------
// Exercise 9: tuples
// Swap the two elements: (7, "seven") becomes ("seven", 7).
// Tip: destructure with `let (number, word) = pair;`

pub fn swap(pair: (i32, String)) -> (String, i32) {
    let (a, b) = pair;
    (b, a)
}

// ---------------------------------------------------------------------------
// Exercise 10: arrays
// Return (first element, middle element, last element).

pub fn first_middle_last(arr: [i32; 5]) -> (i32, i32, i32) {
    (arr[0], arr[2], arr[4])
}

// ---------------------------------------------------------------------------
// Exercise 11: booleans
// A person can vote if they are at least 18 AND a citizen.

pub fn can_vote(age: u8, is_citizen: bool) -> bool {
    is_citizen && age >= 18
}

// ===========================================================================
// Tests: don't edit below this line.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds() {
        assert_eq!(seconds_in_days(1), 86_400);
        assert_eq!(seconds_in_days(7), 604_800);
        assert_eq!(seconds_in_days(0), 0);
    }

    #[test]
    fn price() {
        assert_eq!(final_price(1000, 150, 200), 950);
        assert_eq!(final_price(500, 0, 0), 500);
    }

    #[test]
    fn shadowing() {
        assert_eq!(parse_and_double("  21 "), 42);
        assert_eq!(parse_and_double("-5\n"), -10);
    }

    #[test]
    fn overflow() {
        assert_eq!(add_saturating(250, 10), 255);
        assert_eq!(add_saturating(1, 2), 3);
        assert_eq!(add_wrapping(250, 10), 4);
        assert_eq!(add_wrapping(255, 1), 0);
        assert_eq!(add_checked_or_zero(250, 10), 0);
        assert_eq!(add_checked_or_zero(100, 55), 155);
    }

    #[test]
    fn bill() {
        assert_eq!(split_bill(1000, 3), (333, 1));
        assert_eq!(split_bill(900, 3), (300, 0));
        assert_eq!(split_bill(5, 10), (0, 5));
    }

    #[test]
    fn avg() {
        assert_eq!(average([1.0, 2.0, 3.0, 4.0]), 2.5);
        assert_eq!(average([10.0, 10.0, 10.0, 10.0]), 10.0);
    }

    #[test]
    fn percent() {
        assert_eq!(percentage(1, 4), 25.0);
        assert_eq!(percentage(3, 3), 100.0);
        assert_eq!(percentage(0, 7), 0.0);
    }

    #[test]
    fn chars() {
        assert_eq!(utf8_bytes('a'), 1);
        assert_eq!(utf8_bytes('অ'), 3);
        assert_eq!(utf8_bytes('🦀'), 4);
    }

    #[test]
    fn tuples() {
        assert_eq!(swap((7, String::from("seven"))), (String::from("seven"), 7));
    }

    #[test]
    fn arrays() {
        assert_eq!(first_middle_last([1, 2, 3, 4, 5]), (1, 3, 5));
        assert_eq!(first_middle_last([9, 0, -4, 0, 7]), (9, -4, 7));
    }

    #[test]
    fn voting() {
        assert!(can_vote(18, true));
        assert!(can_vote(40, true));
        assert!(!can_vote(17, true));
        assert!(!can_vote(30, false));
    }
}
