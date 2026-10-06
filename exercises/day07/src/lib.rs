//! Day 7: Week 1 exam. Everything from Days 1–6, mixed together.
//!
//!     cargo test -p day07
//!
//! Try to finish WITHOUT looking back at the lessons first. Look things up only when
//! you're truly stuck, and note in your log what you had to look up. That's your
//! review list.

// Delete this line once every exercise is done.
#![allow(unused_variables)]

// ---------------------------------------------------------------------------
// Q1: Body-mass index = weight (kg) / height (m)²

pub fn bmi(weight_kg: f64, height_m: f64) -> f64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Q2: BMI category as a String
// below 18.5 → "underweight", below 25.0 → "normal", below 30.0 → "overweight", else → "obese"

pub fn bmi_category(bmi: f64) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Q3: the largest of three numbers (no arrays allowed. Use if/else or a mut variable.)

pub fn max_of_three(a: i64, b: i64, c: i64) -> i64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Q4: Bangladeshi taka notes
// Break `amount` into the FEWEST notes, using these denominations:
//     1000, 500, 200, 100, 50, 20, 10, 5, 2, 1
// Return an array with how many of each note, in that order.
// Example: 1789 → 1×1000, 1×500, 1×200, 0×100, 1×50, 1×20, 1×10, 1×5, 2×2, 0×1
//
// Tip: keep a `const NOTES: [u32; 10] = [...]`, a `let mut counts = [0; 10];`,
// and loop over the indexes with `for i in 0..10`.

pub fn taka_notes(amount: u32) -> [u32; 10] {
    todo!()
}

// ---------------------------------------------------------------------------
// Q5: digital root
// Keep summing the digits until a single digit remains.
// 9875 → 9+8+7+5 = 29 → 2+9 = 11 → 1+1 = 2

pub fn digital_root(n: u64) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Q6: count the vowels (a, e, i, o, u, upper or lower case) in a word.
// New (small) thing: `for c in word.chars()` loops over the characters of a text.
// A `match` arm can accept several values: 'a' | 'e' | 'i' => ...

pub fn count_vowels(word: &str) -> u32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Q7: ownership. This compiles, but it's WRONG: it returns an empty label.
// Read it carefully, figure out why, and fix it. (Hint: what does `tag_line` own?)

pub fn label(name: String, role: String) -> String {
    let mut line = String::new();
    tag_line(line.clone(), name, role);
    line
}

fn tag_line(mut line: String, name: String, role: String) -> String {
    line.push_str(&name);
    line.push_str(" - ");
    line.push_str(&role);
    line
}

// ---------------------------------------------------------------------------
// Q8: count down
// Return a String like "3... 2... 1... Liftoff!" for n = 3.
// For n = 0, just "Liftoff!". Use a loop over a REVERSED range: (1..=n).rev()

pub fn countdown(n: u32) -> String {
    todo!()
}

// ===========================================================================
// Tests: don't edit below this line.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_bmi() {
        let v = bmi(70.0, 1.75);
        assert!((v - 22.857).abs() < 0.001, "got {v}");
    }

    #[test]
    fn q2_categories() {
        assert_eq!(bmi_category(17.0), "underweight");
        assert_eq!(bmi_category(18.5), "normal");
        assert_eq!(bmi_category(24.9), "normal");
        assert_eq!(bmi_category(25.0), "overweight");
        assert_eq!(bmi_category(31.2), "obese");
    }

    #[test]
    fn q3_max() {
        assert_eq!(max_of_three(1, 2, 3), 3);
        assert_eq!(max_of_three(3, 2, 1), 3);
        assert_eq!(max_of_three(-1, -7, -3), -1);
        assert_eq!(max_of_three(5, 5, 2), 5);
    }

    #[test]
    fn q4_notes() {
        assert_eq!(taka_notes(1789), [1, 1, 1, 0, 1, 1, 1, 1, 2, 0]);
        assert_eq!(taka_notes(3000), [3, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(taka_notes(0), [0; 10]);
        assert_eq!(taka_notes(8), [0, 0, 0, 0, 0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn q5_root() {
        assert_eq!(digital_root(9875), 2);
        assert_eq!(digital_root(7), 7);
        assert_eq!(digital_root(0), 0);
        assert_eq!(digital_root(999_999_999_999), 9);
    }

    #[test]
    fn q6_vowels() {
        assert_eq!(count_vowels("Rust"), 1);
        assert_eq!(count_vowels("OWNERSHIP"), 3);
        assert_eq!(count_vowels("rhythm"), 0);
        assert_eq!(count_vowels("Bangladesh"), 3);
    }

    #[test]
    fn q7_label() {
        assert_eq!(
            label(String::from("Ayesha"), String::from("Rust engineer")),
            "Ayesha - Rust engineer"
        );
    }

    #[test]
    fn q8_countdown() {
        assert_eq!(countdown(3), "3... 2... 1... Liftoff!");
        assert_eq!(countdown(1), "1... Liftoff!");
        assert_eq!(countdown(0), "Liftoff!");
    }
}
