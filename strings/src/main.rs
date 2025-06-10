fn main() {

    let n = -123;
    let m = 55;

    let mut s = n.to_string();

    println!("{}", &s[1..4]);
    s.push_str(&m.to_string());
    println!("{s}");

    let s1 = "Hello, ".to_string();
    let s2 = "World!".to_string();
    let s3 = s1 + &s2[..1];

    println!("{s2}, {s3}");
    println!("{s}");

    let hi = "أهلا و سهلاً".to_string();

    println!("{}", hi.len());
    // two byte character
    if let Some(c) = hi.chars().rev().nth(3) { // 4th from end
        println!("{}", c);
    }
    println!("{}", &hi[0..2]); // index by bytes

    // or return bytes
    if let Some(slice) = hi.get(0..2) {
        println!("{:?}", slice.as_bytes());
    }

}
