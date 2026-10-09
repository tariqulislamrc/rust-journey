// const SECONDS_PER_HOUR: u32 = 60*60;
// const MAX_LOGIN_ATTEMPTS: u32 = 5;

fn main() {
    // // variables are immutable by default
    // let mut x = 5;
    // println!("x = {x}");
    // x = 6;
    // println!("x = {x}");
    //
    // // Constants
    // println!("{SECONDS_PER_HOUR} seconds per hour, max {MAX_LOGIN_ATTEMPTS} attempts");
    //
    // //shadowing
    // let y = 5;
    //  let y = y + 1;
    // {
    //     let y = y + 2;
    //     println!("inner y = {y}");
    // }
    //
    // println!("outer y = {y}");
    //
    // // shadowing can change the type:
    // let spaces = "       ";
    // let spaces = spaces.len();
    // println!("{spaces}");

    // mut can not change the type
    // let mut count = "        ";
    // count = count.len();

    // let a : u8 = 250;
    // let b : u8 = 10;
    //
    // println!("{}", a.wrapping_add(b));
    // println!("{}", a.saturating_add(b));
    // println!("{:?}", a.checked_add(b));
    // println!("{:?}", a.checked_add(5));
    // println!("{}", a.checked_add(b).unwrap_or(0));
    //
    //
    // let person : (&str, u8, f64) = ("Tariqul", 34, 1.69);
    // println!("{} is {} years old", person.0, person.1);
    //
    // let (name, age, height) = person;
    // println!("{name}, {age}, {height}m");
    //
    //
    // let numbers : [i32; 5] = [1, 2, 3, 4, 5];
    // let zeroes : [i32; 8] = [0;8];
    // let months = ["jan", "feb", "mar", "apr", "jun", "jul", "aug", "sep"];
    //
    // println!("first = {}, length = {}", numbers[0], numbers.len());
    // println!("{:?}", zeroes);
    // println!("{}", months[3]);

    // println!("{}", get([1,2,3], 10));
    println!("{}", 7/2)

}

// fn get(a: [i32; 3], i: usize) -> i32 {
//     a[i]
// }
