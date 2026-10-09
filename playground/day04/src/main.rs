fn main() {
    greet("Tariqul", 9);
    let total = add(2, 3);
    println!("2 + 3 = {total}");

    let temparature = 31;
    if temparature > 30 {
        println!("Hot day");
    } else if temparature > 20 {
        println!("Pleasant");
    } else{
        println!("Cold");
    }

    let score = 72;
    let passed = if score >= 50 { "pass"} else { "fail"};
    println!("{passed}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {result}");

    let mut n = 3;
    while n> 0 {
        println!("{n}...");
        n -= 1;
    }
    println!("Liftoff!");

    let prices = [120, 450, 80];
    let mut total = 0;
    for price in prices {
        total += price;
    }

    println!("total = {total}");

    for i in 1..4 {
        print!("{i} ") ;
    }
    println!();

    for i in 1..=4 {
        print!("{i} ");
    }
    println!();

    for i in (1..=3).rev() {
        print!("{i} ");
    }
    println!();

    for i in (0..10).step_by(3) {
        print!("{i} ");
    }
    println!();

    let names = ["Rahim", "Karim", "Fatima"];
    for (i, name) in names.iter().enumerate() {
        println!("{i}: {name}");
    }

    'tariq: for x in 1..=5 {
        for y in 1..=5 {
            if x * y == 12 {
                println!("found {x} x {y} = 12");
                break 'tariq;
            }
        }
    }

    // multiplacation table

    for row in 1..=10 {
        for col in 1..=10 {
            print!("{:>4}", row * col );
        }
        println!();
    }


}

fn greet (name: &str, years: u32) {
    println!("Hi {name}! {years} years of PHP, day 4 of Rust.");
}

fn add (a: i32, b: i32) -> i32 {
    a + b
}