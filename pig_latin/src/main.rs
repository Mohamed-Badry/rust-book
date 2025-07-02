

fn pig_latin(word: &str) -> Option<String> {
    let mut chars = word.chars();
    let first = chars.next()?;

    if   "aeiouAEIOU".contains(first) {
        Some(format!("{word}-hay"))
    } else {

        let rest: String = chars.collect();
        Some(format!("{rest}-{first}ay"))
    }
}

fn main() {
    
    println!("{:?}", pig_latin("first"));
    println!("{:?}", pig_latin("apple"));
    println!("{:?}", pig_latin("word"));
    println!("{:?}", pig_latin("monkey"));
}
