use chapter14_crate::PrimaryColor;
use chapter14_crate::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mixed = mix(red, yellow);
    println!("{:#?}", mixed);
}