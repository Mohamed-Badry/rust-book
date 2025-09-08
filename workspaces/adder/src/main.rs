use rand;

fn main() {
    let num = rand::random::<u64>() % 100;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    println!("Add a random number to {num}: {}", add_random::add_random(num));
}