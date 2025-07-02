use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn read_username_from_file(file: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(file)?.read_to_string(&mut username)?;
    
    Ok(username)
}

fn main() {

    let username = read_username_from_file("username.txt").unwrap();
    println!("{username}");

    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    let mut line = String::new();
    match file.read_to_string(&mut line) {
        Ok(len) => println!("The text found in the file is {} bytes long.", len),
        Err(error) => print!("{:?}", error),
    }
    println!("{}", line);
}
