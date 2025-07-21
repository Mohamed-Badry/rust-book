fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = find_largest(&number_list);
    println!("The largest number is {largest}");

    let float_list = vec![102.3, 34.1, 60.21, 89.38, 54.90, 2.43, 43.2, 8.0];
    let largest = find_largest(&float_list);
    println!("The largest number is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest(&char_list);
    println!("The largest char is {result}");

    let p = Point::new(4, 6);
    println!("x: {}, y: {}", p.x(), p.y());

    let p2: Point<f32> = Point::new(4.32, 6.21);
    println!("Distance from origin: {}", p2.distance_from_origin());
}
