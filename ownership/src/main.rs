fn main() {
    let s = String::from("tree Hello world monkey");
    let fw = first_word(&s);
    let sw = second_word(&s);
    let lw = last_word(&s);

    println!("{}", fw);
    println!("{}", sw);
    println!("{}", lw);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..s.len()];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && start == 0 {
            start = i + 1;
        } else if item == b' ' && start != 0 {
            return &s[start..i];
        }
    }

    s
}
