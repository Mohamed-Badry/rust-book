fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn make_averager() -> impl FnMut(i32) -> f32 {

    let mut nums: Vec<i32> = vec![];

    let averager = move |num| {
        nums.push(num);
        nums.iter().sum::<i32>() as f32/(nums.len() as f32) 
    };

    averager
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    let mut averager = make_averager();

    assert_eq!(averager(5), 5.0);
    assert_eq!(averager(25), 15.0);
    assert_eq!(averager(15), 15.0);
}