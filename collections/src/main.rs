fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    if let Some(list) = v.get(0..5) {
        for (i, n) in list.iter().enumerate() {
            println!("The {i}th element is {n}");
        }
    } else {
        println!("There is no second element.");
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(does_not_exist) => println!("{does_not_exist}"),
        None => println!("The element doesn't exist."),
    }

    let first = &v[0];

    println!("The first element is: {first}");

    v.push(6);

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
}
