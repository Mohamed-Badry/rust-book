use std::io;

fn main() {
    // prints the first 12 numbers in the fibonacci sequence
    for n in 0..=12 {
        print!("{} ", fibonnaci(n));
    }
    println!();

    // Temperature conversion from F to C and from C to F
    loop {
        println!("Specifiy the scale you want to convert from: F/C");
        let mut from = String::new();

        io::stdin()
            .read_line(&mut from)
            .expect("Failed to read temperature");

        let from: char = from
            .to_lowercase()
            .trim()
            .parse()
            .expect("Failed to convert string to char");

        if from == 'f' {
            prompt_f();
        } else if from == 'c' {
            prompt_c();
        } else {
            println!("Invalid system")
        }
    }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn prompt_f() {
    println!("Enter temperature in Faherenheit: ");

    let mut temp_f = String::new();

    io::stdin()
        .read_line(&mut temp_f)
        .expect("Failed to read temperature");

    let temp_f = temp_f
        .trim()
        .parse()
        .expect("Failed to convert temperature to float");

    let temp_c = f_to_c(temp_f);

    println!("{temp_f}F = {temp_c}C");
}

fn prompt_c() {
    println!("Enter temperature in Celsius: ");

    let mut temp_c = String::new();

    io::stdin()
        .read_line(&mut temp_c)
        .expect("Failed to read temperature");

    let temp_c = temp_c
        .trim()
        .parse()
        .expect("Failed to convert temperature to float");

    let temp_f = c_to_f(temp_c);

    println!("{temp_c}C = {temp_f}F")
}

fn fibonnaci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}
