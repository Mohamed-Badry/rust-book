fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next_back(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), None);
    // assert_eq!(v1_iter., Some(&3));
    // assert_eq!(v1_iter., Some(&2));
    // assert_eq!(v1_iter., Some(&1));
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    println!("{:?}", v1);

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    println!("{:?}", v1)
}

#[test]
fn collect_test() {

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x ^ 2).collect();

    dbg!("{:?}", &v2);
    assert_eq!(v2, vec![3, 0, 1]);

}
