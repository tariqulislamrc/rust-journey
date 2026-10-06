//! Day 6: ownership. ⚠️ This file does NOT compile yet. Fixing it IS the exercise.
//!
//!     cargo test -p day06
//!
//! The compiler will show one error at a time (sometimes a few). Read each error
//! fully, including the `help:` lines, then fix it. Rules:
//!   - Don't change the tests at the bottom.
//!   - Don't change the signatures of `pub` functions.
//!   - Each fix is small, usually 1–2 lines.

// ---------------------------------------------------------------------------
// Fix 1: `name` is moved into the first call to `greeting`, so the second call
// can't use it. Make both greetings work.

pub fn greet_twice(name: String) -> (String, String) {
    let first = greeting(name);
    let second = greeting(name);
    (first, second)
}

fn greeting(name: String) -> String {
    format!("Hello, {name}!")
}

// ---------------------------------------------------------------------------
// Fix 2: after `let s2 = s1;`, which variable owns the String?

pub fn move_and_measure() -> (String, usize) {
    let s1 = String::from("ownership");
    let s2 = s1;
    let len = s1.len();
    (s2, len)
}

// ---------------------------------------------------------------------------
// Fix 3: `print_and_count` takes ownership of `text`, so `text` is gone afterwards.
// Do NOT use .clone() here. Instead, change the private helper so it GIVES THE
// STRING BACK (return a tuple), and update the caller to receive it.

pub fn count_then_shout(text: String) -> String {
    let count = print_and_count(text);
    let shout = text.to_uppercase();
    format!("{shout} ({count} chars)")
}

fn print_and_count(s: String) -> usize {
    println!("{s}");
    s.len()
}

// ---------------------------------------------------------------------------
// Fix 4: this one is about `mut`, not ownership, but you'll hit it constantly.

pub fn build_sentence() -> String {
    let s = String::from("Rust");
    s.push_str(" is fun");
    s
}

// ---------------------------------------------------------------------------
// Fix 5: `a` or `b` is moved into `winner`, so printing them afterwards fails.
// Fix it WITHOUT cloning: think about the ORDER of the lines.

pub fn longer_name(a: String, b: String) -> String {
    let winner = if a.len() >= b.len() { a } else { b };
    println!("Compared {a} and {b}");
    winner
}

// ---------------------------------------------------------------------------
// Fix 6 (no compile error here!): this function already compiles and passes.
// Your task: in the comment below, explain WHY `x` is still usable after
// `let y = x;`, when the same thing with a String (Fix 2) failed.
//
// Your explanation:
//

pub fn copy_numbers() -> (i32, i32) {
    let x = 5;
    let y = x;
    (x, y)
}

// ===========================================================================
// Tests: don't edit below this line.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix1() {
        let (a, b) = greet_twice(String::from("Ayesha"));
        assert_eq!(a, "Hello, Ayesha!");
        assert_eq!(b, "Hello, Ayesha!");
    }

    #[test]
    fn fix2() {
        assert_eq!(move_and_measure(), (String::from("ownership"), 9));
    }

    #[test]
    fn fix3() {
        assert_eq!(count_then_shout(String::from("rust")), "RUST (4 chars)");
    }

    #[test]
    fn fix4() {
        assert_eq!(build_sentence(), "Rust is fun");
    }

    #[test]
    fn fix5() {
        assert_eq!(longer_name(String::from("Ana"), String::from("Ayesha")), "Ayesha");
        assert_eq!(longer_name(String::from("Rahim"), String::from("Kia")), "Rahim");
    }

    #[test]
    fn fix6() {
        assert_eq!(copy_numbers(), (5, 5));
    }
}
