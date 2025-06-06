fn main() {
    let y = {
        let x = 3;
        (x + 1,
        x * 5)
    };

    println!("The value of y is: {}, {}", y.0, y.1);
}