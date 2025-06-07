struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1 = build_user("monkey@jungle.tree", "monkey");

    let mut user2 = User{
        email: String::from("orangutan@branch.tree"),
        ..user1
    };

    println!("{}", user1.active);
    // println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    
    println!("{}", user2.active);
    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
    
    // sing_in(&mut user1);
    println!("{}", user1.sign_in_count);
    
    sing_in(&mut user2);
    println!("{}", user1.sign_in_count);
    println!("{}", user2.sign_in_count);
    
    change_username(&mut user2, "monkey man");
    println!("{}", user2.username);
    
}

fn sing_in(user: &mut User) {
    user.sign_in_count += 1;
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}


fn build_user(email: &str, username: &str) -> User {
    let email = String::from(email);
    let username = String::from(username);
    
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

