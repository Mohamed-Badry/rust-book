use std::{fmt, ops::{Add, Deref, DerefMut, Sub}, vec};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Millimeters(u32);
#[derive(Debug, Clone, Copy)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;

    fn add(self, other: Millimeters) -> Meters {
        Meters(self.0 + (other.0 / 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wrapper {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Point { x: 5, y: 6 } - Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let d1 = Meters(2);
    let d2 = Millimeters(3000);

    println!("d1 + d2 = {:?}", d1 + d2);
    println!("d2 + d1 = {:?}", d2 + d1);


    let human = Human;

    human.fly();
    Wizard::fly(&human);
    Pilot::fly(&human);

    let mut w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
    
    (*w).push(String::from("monkey"));
    println!("w = {w}");

    let f = (*w).first().unwrap();
    println!("{f}");

}