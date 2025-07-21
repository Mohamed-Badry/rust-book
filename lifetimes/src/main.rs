trait HasLength {
    fn len(&self) -> usize;
}

impl HasLength for str {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

fn longest<'a, T: HasLength + ?Sized>(x: &'a T, y: &'a T) -> &'a T {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let result = "";
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string (inner scope) is {result}");
    }    
    println!("The longest string (outer scope) is {result}");

    let vec1 = vec![1, 52, 3, 1, 3];
    let vec2 = vec![4, 2, 2];

    let result = longest(&vec1, &vec2);
    println!("The longest vector is {result:?}");
}
