#[macro_export]
macro_rules! monk {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    
}

#[allow(unused_mut)]
fn main() {
    assert_eq!(monk![1, 2, 3, 4], vec![1, 2, 3, 4]);
    assert_eq!(monk!["String"], vec!["String"]);
    
    let m: Vec<f32> = monk![];
    let v: Vec<f32> = vec![];

    assert_eq!(m, v);
}
